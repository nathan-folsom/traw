# T(erminalD)raw

Terminal ui with vim-inspired keybindings for drawing system diagrams

## Basic Usage

Call from the command line:
```bash
traw [options]
```

### Options

- file_name (optional, defaults to 'unnamed.traw') file to read from/save to

## Keybindings

- 'q' (normal mode) - Exit `traw`
- 'i' (normal mode) - Enter draw mode
  - Start drawing a box when cursor is in empty space
  - Start drawing an arrow when cursor is on a box boundary
  - Edit box text content when cursor is inside a box
- 'r' (normal mode) - Edit an existing box by dragging from a corner
- 'hjkl' (normal mode, draw mode) - Move around
- 'x' (normal mode) - Delete shape under cursor
- 's' (normal mode) - Save current file
- 'v' (normal mode) - Enter select mode
- 'y' (select mode) - Copy selection to system clipboard
- 'enter' - Transition to next mode
  - (select mode) Copy selection and end select
  - (draw mode) End draw mode
- 'd' (normal mode) - Toggle debug panel


## File 

## Feature To-do

Things that are useful and should be added:

- Count modifiers: e.g. '10l' to go 10 cols to the right
- Word motions: e.g. 'w' to go to start of next object
- Infinite canvas: i.e. terminal is a window onto a larger space, instead of representing the whole space.
- Toggle debug pane
- Move boxes
- Configurable keybindings
- Background grid: i.e. show some sort of guidelines
- Use junction characters where arrows intersect boxes
- Box text justification and alignment
- Flexboxy layout
- Undo/redo
- Command line docs
- Build/distribution
- Hover effects: e.g. change char under cursor if action is available
- Animations: e.g. hover background transition instead of instant change
- Color theme
