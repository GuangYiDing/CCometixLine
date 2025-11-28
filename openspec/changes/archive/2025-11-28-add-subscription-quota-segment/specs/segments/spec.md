## ADDED Requirements
### Requirement: Subscription Quota Segment
The system SHALL provide a new segment type called "SubscriptionQuota" that displays subscription quota information using an external script to fetch data from any subscription service.

#### Scenario: Basic percentage display
- **WHEN** the segment is enabled and configured with a script that outputs "75"
- **THEN** the segment displays "75%" with the corresponding circle icon (f0aa2 for 51-62% range)

#### Scenario: JSON format parsing
- **WHEN** the segment is configured with output_format = "json" and the script outputs {"percentage": 82.5, "remaining": "$18.00", "total": "$100.00"}
- **THEN** the segment displays "83%" (rounded) with the circle icon for 76-87% range, and secondary text "· $18.00"

#### Scenario: Key-value pair parsing
- **WHEN** the segment is configured with output_format = "key_value" and the script outputs "percentage=45"
- **THEN** the segment displays "45%" with the circle icon for 38-50% range

#### Scenario: Script execution with cache
- **WHEN** the segment is configured with cache_duration = 300 seconds and a script path
- **THEN** the system SHALL execute the script no more than once every 300 seconds
- **AND** SHALL use cached data if available and not expired

#### Scenario: Script execution failure fallback
- **WHEN** the configured script fails to execute or returns invalid data
- **THEN** the system SHALL display "-" as the primary text
- **AND** SHALL log an error message
- **AND** SHALL attempt to use cached data if available

#### Scenario: Circle icon progression
- **WHEN** the percentage value is 0-12%
- **THEN** display icon U+F0A9E (circle_slice_1)
- **WHEN** the percentage value is 13-25%
- **THEN** display icon U+F0A9F (circle_slice_2)
- **WHEN** the percentage value is 26-37%
- **THEN** display icon U+F0AA0 (circle_slice_3)
- **WHEN** the percentage value is 38-50%
- **THEN** display icon U+F0AA1 (circle_slice_4)
- **WHEN** the percentage value is 51-62%
- **THEN** display icon U+F0AA2 (circle_slice_5)
- **WHEN** the percentage value is 63-75%
- **THEN** display icon U+F0AA3 (circle_slice_6)
- **WHEN** the percentage value is 76-87%
- **THEN** display icon U+F0AA4 (circle_slice_7)
- **WHEN** the percentage value is 88-100%
- **THEN** display icon U+F0AA5 (circle_slice_8)

### Requirement: Custom Script Configuration
The system SHALL support configuring external scripts to fetch quota data through the following options:

#### Scenario: Script path configuration
- **WHEN** user sets custom_script_path in segment options
- **THEN** the system SHALL execute the script at the specified path
- **AND** SHALL use the script's stdout as the data source
- **AND** SHALL timeout script execution after configured timeout period (default 5 seconds)

#### Scenario: Output format configuration
- **WHEN** user selects output_format option with value "text"
- **THEN** the system SHALL parse the script output as plain text containing only a numeric percentage
- **WHEN** user selects output_format option with value "json"
- **THEN** the system SHALL parse the script output as JSON and extract the "percentage" field
- **WHEN** user selects output_format option with value "key_value"
- **THEN** the system SHALL parse the script output as "key=value" pairs and extract the "percentage" key

#### Scenario: Parse rules configuration
- **WHEN** user configures custom parse rules in JSON format
- **THEN** the system SHALL use these rules to extract percentage from script output
- **AND** SHALL support regex patterns for extraction
- **AND** SHALL support field path notation for JSON extraction

### Requirement: TUI Configuration Integration
The system SHALL provide TUI configuration interface for the SubscriptionQuota segment.

#### Scenario: Segment list shows SubscriptionQuota
- **WHEN** user navigates to segment list in TUI config
- **THEN** the SubscriptionQuota segment SHALL appear in the list
- **AND** SHALL be disabled by default
- **AND** SHALL show as "Subscription Quota" in the UI

#### Scenario: Segment editor shows custom options
- **WHEN** user selects SubscriptionQuota segment in editor
- **THEN** the editor SHALL display standard options (icon, colors, style)
- **AND** SHALL display custom options section with:
  - Script path input field
  - Output format dropdown (text/json/key_value)
  - Cache duration input
  - Timeout duration input
  - Parse rules text area (optional)

### Requirement: Default Configuration
The system SHALL provide default configuration for the SubscriptionQuota segment.

#### Scenario: Default segment state
- **WHEN** creating a new configuration
- **THEN** the SubscriptionQuota segment SHALL be present but disabled (enabled = false)
- **AND** SHALL use default icon configuration
- **AND** SHALL use default colors from the active theme

#### Scenario: Default options
- **WHEN** SubscriptionQuota segment is enabled
- **THEN** the following default options SHALL be set:
  - cache_duration: 300 (5 minutes)
  - timeout: 5 (seconds)
  - output_format: "text"
  - custom_script_path: "" (empty, user must configure)
