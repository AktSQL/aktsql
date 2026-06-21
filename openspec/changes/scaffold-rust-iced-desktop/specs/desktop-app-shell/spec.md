## ADDED Requirements

### Requirement: Desktop app launches
The system SHALL provide a Rust desktop application named AktSQL Database Management that launches with iced.

#### Scenario: Launch application
- **WHEN** the user runs the desktop application
- **THEN** a window opens with Akt branding

### Requirement: Workbench shell is visible
The desktop application SHALL display a database-manager workbench shell with a top bar, left navigation, central workspace, and bottom status bar.

#### Scenario: View initial shell
- **WHEN** the desktop application opens
- **THEN** the user sees top navigation, sidebar navigation, workspace content, and status information

### Requirement: Theme foundation exists
The desktop application SHALL include dark and light theme modes with Akt red/black branding as the default visual direction.

#### Scenario: Toggle theme mode
- **WHEN** the user activates the theme toggle
- **THEN** the application switches between dark and light theme state without closing the window

### Requirement: Navigation shell is stateful
The desktop application SHALL track the selected navigation area in application state.

#### Scenario: Select navigation item
- **WHEN** the user selects a sidebar navigation item
- **THEN** the selected item is visually indicated and the workspace title changes to match it

### Requirement: Verification commands are documented
The repository SHALL document how to run, format, and check the Rust iced application.

#### Scenario: Read repository commands
- **WHEN** a developer opens the repository documentation
- **THEN** they can find commands to run the desktop app and perform basic Rust verification
