import { explain } from './pkg';

// add event on document ready
function init() {
    console.log('init');

    const mainInput = document.getElementById('mainInput')
    const mainOutput = document.getElementById('mainOutput')
    const updateExplanation = function() {
        mainOutput.innerHTML = explain(mainInput.value)
    }

    mainInput.addEventListener('keydown', updateExplanation)
    mainInput.addEventListener('keyup', updateExplanation)
    updateExplanation()
}

init();