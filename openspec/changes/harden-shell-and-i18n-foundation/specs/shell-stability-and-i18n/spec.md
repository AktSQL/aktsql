## ADDED Requirements

### Requirement: Fixed Native Window Shell
The system SHALL present Akt in a fixed-size desktop window while preserving standard native close and minimize behavior.

#### Scenario: Application window opens
- **WHEN** the desktop application starts
- **THEN** the window uses the Akt application title
- **THEN** the window opens at the fixed product size
- **THEN** the window keeps native decorations enabled

#### Scenario: User minimizes the window
- **WHEN** the user activates the native minimize control
- **THEN** the operating system receives a standard minimize request

### Requirement: Bounded Connection Manager Layout
The system SHALL constrain connection manager lists, forms, validation messages, and status text so they fit within the fixed application window.

#### Scenario: User opens the connection manager
- **WHEN** the Databases workspace is selected
- **THEN** the profile list has an explicit fixed width
- **THEN** the editable form has an explicit maximum width
- **THEN** overflowing vertical form content remains reachable through scrolling

#### Scenario: Long profile or driver labels are shown
- **WHEN** a profile name, target, driver label, validation message, or status message is longer than its available container
- **THEN** the UI displays a bounded shortened label instead of resizing the container

### Requirement: Language-Ready Text Boundaries
The system SHALL keep active shell and connection-manager user-facing copy in a centralized text catalog before runtime language switching is introduced.

#### Scenario: Shell text is rendered
- **WHEN** top navigation, side navigation, status text, connection form labels, or connection actions are rendered
- **THEN** the text is read from the i18n text catalog or a typed domain label

### Requirement: Stable Connection Config Persistence
The system SHALL persist saved connection profiles to a stable Akt configuration file without storing passwords.

#### Scenario: Profiles are saved
- **WHEN** the user saves or deletes a connection profile
- **THEN** the app writes the connection profile list to the preferred Akt config path
- **THEN** saved profile data does not include the password field

#### Scenario: Existing development config is present
- **WHEN** the preferred Akt config path is empty and `aktsql.config.json` exists in the working directory
- **THEN** the app loads the working-directory config as a fallback
