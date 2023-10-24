const baseTitle = document.title;

export function setTitle(view = "", subject = "") {
    let title = baseTitle;
    if (view) {
        title += " " + view;
    }
    if (subject) {
        title += " " + subject;
    }
    document.title = title;
}
