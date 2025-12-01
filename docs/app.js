// XPlit Web Application
// Handles WASM module loading and UI interactions

let wasmModule = null;

// Initialize WASM module
async function initWasm() {
    try {
        const { default: init, wasm_split, wasm_recover } = await import('./pkg/xplit.js');
        await init();

        wasmModule = {
            split: wasm_split,
            recover: wasm_recover
        };

        console.log('WASM module loaded successfully');
        enableForms();
    } catch (error) {
        console.error('Failed to load WASM module:', error);
        showGlobalError('Failed to load WASM module. Please refresh the page.');
    }
}

// Enable forms after WASM loads
function enableForms() {
    const splitForm = document.getElementById('split-form');
    const recoverForm = document.getElementById('recover-form');

    splitForm.addEventListener('submit', handleSplit);
    recoverForm.addEventListener('submit', handleRecover);
}

// Handle split form submission
async function handleSplit(event) {
    event.preventDefault();

    const secretInput = document.getElementById('secret-input').value;

    const splitError = document.getElementById('split-error');
    const splitResult = document.getElementById('split-result');

    // Clear previous results
    splitError.style.display = 'none';
    splitResult.style.display = 'none';

    // Validate input
    if (!secretInput || secretInput.trim().length === 0) {
        showError(splitError, 'Please enter a secret');
        return;
    }

    // Show loading state
    const submitBtn = event.target.querySelector('button[type="submit"]');
    const originalBtnText = submitBtn.textContent;
    submitBtn.textContent = 'Generating...';
    submitBtn.disabled = true;

    try {
        // Call WASM split function
        const resultJson = wasmModule.split(secretInput);
        const result = JSON.parse(resultJson);

        // Display shares
        document.getElementById('share1-display').textContent = result.share1;
        document.getElementById('share2-display').textContent = result.share2;

        // Store shares globally for copy/QR functions
        window.currentShares = {
            share1: result.share1,
            share2: result.share2
        };

        // Clear any existing QR codes
        document.getElementById('qr-code-1').innerHTML = '';
        document.getElementById('qr-code-2').innerHTML = '';
        document.getElementById('qr-container-1').style.display = 'none';
        document.getElementById('qr-container-2').style.display = 'none';

        splitResult.style.display = 'block';

    } catch (error) {
        showError(splitError, `Split failed: ${error.message || error}`);
    } finally {
        submitBtn.textContent = originalBtnText;
        submitBtn.disabled = false;
    }
}

// Handle recover form submission
async function handleRecover(event) {
    event.preventDefault();

    const share1Input = document.getElementById('share1-input').value.trim();
    const share2Input = document.getElementById('share2-input').value.trim();

    const recoverError = document.getElementById('recover-error');
    const recoverResult = document.getElementById('recover-result');

    // Clear previous results
    recoverError.style.display = 'none';
    recoverResult.style.display = 'none';

    if (!share1Input || !share2Input) {
        showError(recoverError, 'Please enter both shares');
        return;
    }

    // Show loading state
    const submitBtn = event.target.querySelector('button[type="submit"]');
    const originalBtnText = submitBtn.textContent;
    submitBtn.textContent = 'Recovering...';
    submitBtn.disabled = true;

    try {
        // Call WASM recover function
        const recoveredSecret = wasmModule.recover(share1Input, share2Input);

        // Display recovered secret
        const secretElement = document.getElementById('recovered-secret');
        secretElement.textContent = recoveredSecret;
        secretElement.classList.remove('revealed');

        recoverResult.style.display = 'block';

    } catch (error) {
        showError(recoverError, `Recovery failed: ${error.message || error}`);
    } finally {
        submitBtn.textContent = originalBtnText;
        submitBtn.disabled = false;
    }
}

// Copy share to clipboard
window.copyShare = async function(shareNumber, buttonElement) {
    const shareText = window.currentShares[`share${shareNumber}`];
    if (!shareText) {
        alert('No share available to copy');
        return;
    }

    const button = buttonElement;

    try {
        await navigator.clipboard.writeText(shareText);
        const originalText = button.textContent;
        button.textContent = 'Copied!';
        setTimeout(() => {
            button.textContent = originalText;
        }, 2000);
    } catch (err) {
        console.error('Clipboard API failed, trying fallback:', err);

        // Fallback for older browsers or permission issues
        const textarea = document.createElement('textarea');
        textarea.value = shareText;
        textarea.style.position = 'fixed';
        textarea.style.opacity = '0';
        document.body.appendChild(textarea);
        textarea.select();

        try {
            const successful = document.execCommand('copy');
            if (successful) {
                const originalText = button.textContent;
                button.textContent = 'Copied!';
                setTimeout(() => {
                    button.textContent = originalText;
                }, 2000);
            } else {
                alert('Failed to copy to clipboard');
            }
        } catch (execErr) {
            console.error('Fallback copy failed:', execErr);
            alert('Failed to copy to clipboard: ' + execErr.message);
        }

        document.body.removeChild(textarea);
    }
};

// Toggle QR code display
window.toggleQR = function(shareNumber, buttonElement) {
    const container = document.getElementById(`qr-container-${shareNumber}`);
    const qrDiv = document.getElementById(`qr-code-${shareNumber}`);
    const button = buttonElement;

    if (container.style.display === 'none') {
        // Show and generate QR code
        container.style.display = 'block';

        // Only generate if not already generated
        if (qrDiv.innerHTML === '') {
            const shareText = window.currentShares[`share${shareNumber}`];

            try {
                new QRCode(qrDiv, {
                    text: shareText,
                    width: 256,
                    height: 256,
                    colorDark: "#000000",
                    colorLight: "#ffffff",
                    correctLevel: QRCode.CorrectLevel.M
                });
            } catch (error) {
                qrDiv.innerHTML = '<p style="color: red;">Failed to generate QR code</p>';
            }
        }

        button.textContent = 'Hide QR Code';
    } else {
        // Hide QR code
        container.style.display = 'none';
        button.textContent = 'Show QR Code';
    }
};

// Show error message
function showError(element, message) {
    element.textContent = message;
    element.style.display = 'block';
}

// Show global error (for WASM loading failures)
function showGlobalError(message) {
    const banner = document.createElement('div');
    banner.className = 'notification is-danger';
    banner.style.position = 'fixed';
    banner.style.top = '1rem';
    banner.style.left = '50%';
    banner.style.transform = 'translateX(-50%)';
    banner.style.zIndex = '1000';
    banner.style.maxWidth = '90%';
    banner.textContent = message;
    document.body.appendChild(banner);
}

// Initialize on page load
initWasm();
