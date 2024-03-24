import "./style.scss";

async function onLanguageChange(event: Event) {
  const internationalizationHandler = await import("./locale");
  await internationalizationHandler.handleLanguageChange(event);
}

async function onThemeChange(event: Event) {
  const themeHandler = await import("./theme");
  themeHandler.handleThemeChange(event);
}

function main() {
  document
    .getElementById("change-language")
    ?.addEventListener("change", (event: Event) => {
      onLanguageChange(event).catch((error: unknown) => {
        console.error(error);
      });
    });
  document
    .getElementById("change-theme")
    ?.addEventListener("change", (event: Event) => {
      onThemeChange(event).catch((error: unknown) => {
        console.error(error);
      });
    });
  // Following this we no longer need the submit settings button
  //document.getElementById("settings-submit-button")?.remove();
}

addEventListener("DOMContentLoaded", main);
