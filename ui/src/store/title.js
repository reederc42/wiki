const baseTitle = document.title;

export function setView(view) {
    let title = baseTitle;
    if (view) {
        title += " " + view;
    }
    document.title = title;
}
