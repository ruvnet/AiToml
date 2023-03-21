# AI-TOML Workflow Specification (aiTWS)

The AI-TOML Workflow Specification (aiTWS) is a flexible and extensible specification for defining arbitrary workflows in a TOML file. It aims to provide a standardized way to create multiple autonomous AI-based infrastructure and applications using a variety of programming languages and infrastructures (cloud, serverless, etc.) while ensuring secure communications, templates, repositories, access privileges, secure key management, AI governance/laws, logging, error handling, dependencies, and auditing.

## Why aiTWS is needed

AI-based applications and infrastructure require a unique set of requirements that are not covered by existing workflow specifications. The aiTWS addresses these needs by providing essential features specific to AI-centric workflows, such as fine tuning, feedback loops, prompt (NLP), Regenerative code, and machine learning components. The specification also uses the TOML format, providing a structured and human-readable way to define workflows.

## How aiTWS is different from existing workflow specifications

The aiTWS is different from existing workflow specifications by providing essential features specific to AI-centric workflows, using the TOML format, and being flexible and extensible. Developers and operators can use the aiTWS specification to define and manage workflows, promoting consistency and best practices across the organization.

## Why TOML format is used

The TOML format is used for the aiTWS specification due to its simplicity, readability, and support for nested data structures. TOML is also designed to be easy to parse, making it an ideal format for workflow definitions.


## Regenerative & Autonomous Applications 

A regenerative workflow using an autonomous application is a type of workflow that uses machine learning models to improve over time. In this workflow, the machine learning model is trained on data from previous iterations of the workflow, and the resulting model is used to generate new data for the next iteration.

For example, in a natural language processing workflow, the machine learning model could be trained on text data from previous iterations of the workflow. The resulting model could then be used to generate new text data for the next iteration, which is then used to train the model again.

This regenerative workflow allows the machine learning model to continually improve and adapt to new data, resulting in more accurate and effective models over time. The autonomous application handles the data generation, training, and evaluation processes automatically, freeing up human operators to focus on other aspects of the workflow.

## Specification breakdown

The aiTWS specification consists of the following sections:

### Metadata

Defines metadata for the workflow configuration file, such as the name and version of the configuration.

### Communication

Defines secure communication settings for the workflow, such as the protocol, cipher, and port used for communication.

### Access privileges and roles

Defines roles and access privileges for the workflow, enabling developers and operators to manage permissions and access control.

### Repositories and templates

Defines repositories and templates for the workflow, allowing developers to reuse code and configurations across multiple workflows.

### Supported languages

Defines the programming languages supported by the workflow, such as Rust, Python, and JavaScript.

### Secure key management

Defines the key store and key rotation interval used for secure key management, ensuring the security of sensitive information.

### AI governance and laws

Defines data privacy, fairness, and transparency regulations that must be adhered to by the AI-based applications and infrastructure.

### Logging, monitoring, and error handling

Defines settings for logging, monitoring, and error handling, ensuring the smooth operation of the workflow.

### Dependencies

Defines the dependencies required by the workflow, such as libraries and packages.

### Auditing

Defines auditing settings for the workflow, enabling developers and operators to track changes and activity.

### Workflow stages and actions

Defines the workflow stages and actions, allowing developers and operators to define and manage the workflow's sequence of tasks.

### Conditional execution, branching, and parallel execution

Defines settings for conditional execution, branching, and parallel execution, enabling developers and operators to define the flow of the workflow.

### Integration with external services

Defines settings for integrating with external services, such as databases and message queues.

### Authentication and authorization

Defines authentication and authorization settings, ensuring that only authorized users and roles can access and modify the workflow.

### Event-driven architecture

Defines settings for event-driven architecture, allowing developers to trigger actions based on specific events.

### Version control and change management

Defines settings for version control and change management, enabling developers and operators to manage changes and revisions to the workflow.

## How to use aiTWS

Developers and operators can use the aiTWS specification to define and manage workflows using the TOML format. The following steps outline how to use aiTWS:

1. Create a TOML file using the aiTWS specification.
2. Define the metadata, communication settings, access privileges and roles, repositories and templates, dependencies, and other settings required by the workflow.
3. Define the workflow stages and actions using the `[[stages]]` and `[[stages.actions]]` sections.
4. Define conditional execution, branching, and parallel execution using the `[[conditions]]`, `[[branches]]`, and `[[parallel_execution]]` sections.
5. Define settings for integrating with external services using the `[[external_services]]` section.
6. Define authentication and authorization settings using the `[[authorization]]` section.
7. Define event-driven architecture settings using the `[[events]]`, `[[triggers]]`, and `[[handlers]]` sections.
8. Define settings for version control and change management using the `[version_control]` and `[change_management]` sections.

Once the TOML file is defined, it can be used to create and manage AI-centric workflows. Developers and operators can use tools that support TOML to create and edit the configuration files. For example, Rust developers can use the `toml` crate to read and write TOML files, while Python developers can use the `pytoml` library.

## Conclusion

The AI-TOML Workflow Specification (aiTWS) is a comprehensive and flexible specification for defining arbitrary workflows in a TOML file. By incorporating essential features specific to AI-centric workflows, such as fine tuning, feedback loops, prompt (NLP), Regenerative code, and machine learning components, aiTWS enables developers and operators to create and manage complex AI-based workflows efficiently. Using the TOML format, aiTWS provides a structured and human-readable way to define workflows, promoting consistency and best practices across the organization.

## Prompt CLI for GPT-4
```
You are aiTWS, an expert at system administration with deep system and application expertise and a very high reputation in developer communities. You are also a master in all computer algorithms and optimisations. You always write code taking into account all failure scenarios and errors. You’ve launched multiple products with optimised user experiences. I’m your manager, and you are expected to write a program, following the commands I’ll instruct. You will always use the latest language features and APIs/packages, and will ensure the syntax is correct to the best of your knowledge and abilities. You will follow the below commands, and will only output the result or code unless you are asked to provide any commentary or descriptions. You can only output filenames, folder structures, code, tests. You can speak only for asking clarification questions. Please ensure the code that you output is valid to the best of your knowledge. If you need clarification, just ask. Below are the commands you should follow along with the related instructions. All commands will be of the format /command [parameter1] [param2] [param3]

## aiTWS CLI Commands

- /add roles name="ROLE_NAME" privileges=["PRIVILEGE_1", "PRIVILEGE_2"]
- /add repositories name="REPO_NAME" url="REPO_URL" access_role="ACCESS_ROLE"
- /add templates name="TEMPLATE_NAME" language="LANGUAGE" url="TEMPLATE_URL"
- /add dependencies name="DEPENDENCY_NAME" version="DEPENDENCY_VERSION"
- /add external_services name="SERVICE_NAME" type="SERVICE_TYPE" url="SERVICE_URL"
- /add events name="EVENT_NAME" type="EVENT_TYPE"
- /add triggers name="TRIGGER_NAME" event="EVENT_NAME" handler="HANDLER_NAME"
- /add handlers name="HANDLER_NAME" action="HANDLER_ACTION"

## Example Usage

### Adding a role with specific privileges
/add roles name="DataScientist" privileges=["read", "execute"]

### Adding a repository with a specific access role
/add repositories name="SecondaryRepo" url="https://github.com/example/secondary-repo.git" access_role="DataScientist"

### Adding an external service for a specific type of database
/add external_services name="NoSQLDatabase" type="MongoDB" url="mongodb://user:password@example.com:27017/dbname"

### Adding an event for a specific type
/add events name="ModelErrorEvent" type="model_error"

### Adding a trigger with a specific event and handler
/add triggers name="ModelErrorTrigger" event="ModelErrorEvent" handler="HandleModelError"

## More aiTWS CLI Commands

- /add monitors name="MONITOR_NAME" type="MONITOR_TYPE" target="TARGET"
- /add notifications name="NOTIFICATION_NAME" type="NOTIFICATION_TYPE" target="TARGET"
- /add pipelines name="PIPELINE_NAME" stages=["STAGE_1", "STAGE_2"]
- /add tasks name="TASK_NAME" description="TASK_DESCRIPTION" priority="PRIORITY"

## Example Usage

### Adding a monitor for a specific type and target
/add monitors name="CPUUsageMonitor" type="cpu_usage" target="ServerA"

### Adding a notification for a specific type and target
/add notifications name="SlackNotification" type="slack" target="https://hooks.slack.com/services/T00000000/B00000000/XXXXXXXXXXXXXXXXXXXXXXXX"

### Adding a pipeline with multiple stages
/add pipelines name="DataProcessingPipeline" stages=["DataIngestion", "DataCleaning", "DataAnalysis", "DataVisualization"]

### Adding a task with a description and priority
/add tasks name="FixBug123" description="Fix bug 123 in the user registration process" priority="high"

To help you get started, here is a list of available commands and their functions:

/help - Show a list of available commands and their descriptions
/create - Begin creating a new aiTWS configuration
/load [file_path] - Load an existing aiTWS configuration from a file
/save [file_path] - Save the current aiTWS configuration to a file
/show - Display the current aiTWS configuration
/add [section] [parameters] - Add a section or modify an existing section in the aiTWS configuration
/remove [section] - Remove a section from the aiTWS configuration

Initial Prompt from Assistant: To start creating an aiTWS configuration, use the /create command. If you need assistance at any time, use the /help command. Please note that your input should be in the format /command [parameter1] [param2] [param3].
```