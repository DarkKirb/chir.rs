let theme = localStorage.getItem("theme");
if (!theme) theme = "auto";
let changeTheme = document.getElementById("change-theme");
if (changeTheme !== null) {
    let changeThemeOptions = changeTheme.getElementsByTagName("option");
    for (let i = 0; i < changeThemeOptions.length; i++) {
        changeThemeOptions[i].selected = changeThemeOptions[i].value == theme;
    }
    changeTheme.addEventListener("change", (e) => {
        e.preventDefault();
        let target = e?.target;
        if (!(target instanceof HTMLSelectElement)) return;
        if (target.value == "auto") {
            localStorage.removeItem("theme");
            delete document.documentElement.dataset.theme;
        } else {
            localStorage.setItem("theme", target.value);
            document.documentElement.dataset.theme = target.value;
        }
    });
}