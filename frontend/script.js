// script.js
const squares = document.querySelectorAll('.square');
const input = document.getElementById('input');
const notificationBox = document.getElementById('notification-box');
attemptCounter = 0;

window.onload = function() {
		fetch('/get_new_word', {
            method: 'GET',
        });
};


input.addEventListener('keyup', async (event) => {
    if (event.key === 'Enter') {
        // Prevent the default behavior of the Enter key (e.g., line break in text area)
        event.preventDefault();

        const text = event.target.value;
        // Send the input to the Rust server
        const response = await fetch('/process_input', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({ input: text }),
        });
		
        if (response.ok) {
            if (response.status === 200) {
                // Handle the response from the Rust server if needed
                const result = await response.json();
                console.log('Server response:', result);
                
                const letters = result["processed_word"].split('').slice(0, 5);
				const guess_mask = result["guess_mask"];
				const selectedSquares = Array.from(squares).slice(attemptCounter*5, attemptCounter*5 + 5);
                selectedSquares.forEach((square, index) => {
                    square.textContent = letters[index] || '';
                    square.style.backgroundColor = guess_mask[index];
                });
                attemptCounter++;
                input.value = '';
            } else {
            const responseText = await response.text();
            console.log('Server response:', responseText);
            notificationBox.textContent = responseText.replace(/["']/g, '');
            notificationBox.style.display = 'block';

            setTimeout(() => {
                notificationBox.style.display = '';
            }, 2000);
            }
        }
    }
});


