# Wiki UI Spec

UI end-to-end tests

## Tests

### View subject list

Steps:

1. Visit homepage
1. See list of subjects

### Edit existing subject from homepage

Steps:

1. Visit homepage
1. Visit random subject
1. See cannot edit
1. Sign in/up
1. Edit subject
1. Save subject
1. See changes
1. Visit home
1. Visit same subject
1. See changes

### Create new subject from homepage

Steps:

1. Visit homepage
1. Create new subject
1. See cannot edit
1. Sign in/up
1. Edit subject
1. Save subject
1. See changes
1. Visit home
1. See new subject
1. Visit same subject
1. See content

### Create existing subject directly

Steps:

1. Visit create new with existing subject
1. See redirect

### Create new subject directly

Steps:

1. Visit create new with new subject
1. See cannot edit
1. Sign in/up
1. Add new content
1. See changes
1. Save has no error
1. Visit homepage
1. See new subject

### Create existing subject from homepage

Steps:

1. Visit create new
1. See cannot edit
1. Sign in/up
1. Add existing title
1. Save
1. See error

### Reload preserves sign in state

Steps:

1. Visit homepage
1. Sign in/up
1. Reload
1. See signed in
1. Sign out
1. Reload
1. See signed out

### Signed out after reload after expiration

Steps:

1. Visit homepage
1. Sign in/up
1. Wait until expiration
1. Reload
1. See signed out

### Signed out after save after expiration

Steps:

1. Visit create new subject
1. Sign in
1. Add new content
1. Save
1. Add more new content
1. Wait until expiration
1. Save
1. See signed out
