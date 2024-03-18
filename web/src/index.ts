import './style.scss';

async function onLanguageChange(event: Event) {
    let internationalizationHandler = await import('./locale');
    await internationalizationHandler.handleLanguageChange(event);
}

async function main() {
    document.getElementById('change-language')?.addEventListener('change', onLanguageChange);
}


addEventListener('DOMContentLoaded', main);