document.addEventListener('DOMContentLoaded', () => {
    const generateKeyButton = document.getElementById('generateKey');
    
    generateKeyButton.addEventListener('click', async () => {
        const response = await browser.runtime.sendMessage({
            method: 'generate_keypair'
        });
        
        if (response.success) {
            alert("Key generated successfully!")
        } else {
            alert("Failed to generate key: " + response.error)
        }
    });
});