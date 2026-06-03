pub(super) fn run_completion_command(args: &[String]) -> std::io::Result<i32> {
    let Some(shell) = args.first().map(|arg| arg.as_str()) else {
        print_completion_help();
        return Ok(2);
    };

    match shell {
        "zsh" => {
            print!("{}", generate_zsh_completion());
            Ok(0)
        }
        "help" | "--help" | "-h" => {
            print_completion_help();
            Ok(0)
        }
        _ => {
            eprintln!("error: unsupported shell: {shell}");
            eprintln!("supported shells: zsh");
            eprintln!();
            print_completion_help();
            Ok(2)
        }
    }
}

fn print_completion_help() {
    eprintln!("herdr completion commands:");
    eprintln!("  herdr completion zsh    Generate zsh completion script");
    eprintln!();
    eprintln!("Installation:");
    eprintln!("  zsh:  herdr completion zsh > ~/.herdr_completion.zsh");
    eprintln!("        Add to ~/.zshrc: source ~/.herdr_completion.zsh");
}

fn generate_zsh_completion() -> &'static str {
    r#"#compdef herdr

_herdr() {
    local -a commands
    local context state state_descr line
    typeset -A opt_args

    _arguments -C \
        '(- *)'{-h,--help}'[Show help information]' \
        '(- *)'{-V,--version}'[Show version information]' \
        '(- *)--default-config[Print default configuration]' \
        '--session[Specify session name]:session:' \
        '--no-session[Run without session]' \
        '--remote[Connect to remote host]:target:' \
        '1: :_herdr_commands' \
        '*::arg:->args'

    case "$state" in
        args)
            case $line[1] in
                agent)
                    _herdr_agent
                    ;;
                channel)
                    _herdr_channel
                    ;;
                completion)
                    _herdr_completion
                    ;;
                config)
                    _herdr_config
                    ;;
                integration)
                    _herdr_integration
                    ;;
                pane)
                    _herdr_pane
                    ;;
                server)
                    _herdr_server
                    ;;
                session)
                    _herdr_session
                    ;;
                status)
                    _herdr_status
                    ;;
                tab)
                    _herdr_tab
                    ;;
                wait)
                    _herdr_wait
                    ;;
                workspace)
                    _herdr_workspace
                    ;;
                worktree)
                    _herdr_worktree
                    ;;
            esac
            ;;
    esac
}

_herdr_commands() {
    local -a commands
    commands=(
        'agent:Manage AI coding agents'
        'channel:Manage update channel (stable/preview)'
        'completion:Generate shell completion scripts'
        'config:Configuration commands'
        'integration:Integration asset commands'
        'pane:Manage terminal panes'
        'server:Server control commands'
        'session:Session management'
        'status:Show server and client status'
        'tab:Manage workspace tabs'
        'update:Download and install the latest version'
        'wait:Wait for conditions'
        'workspace:Manage workspaces'
        'worktree:Manage git worktrees'
    )
    _describe 'command' commands
}

_herdr_agent() {
    local -a subcommands
    subcommands=(
        'list:List all agents'
        'get:Get agent details'
        'read:Read agent output'
        'send:Send input to agent'
        'rename:Rename an agent'
        'focus:Focus an agent'
        'wait:Wait for agent status'
        'attach:Attach to agent'
        'start:Start a new agent'
    )

    _arguments -C \
        '1: :->subcmd' \
        '*::arg:->args'

    case "$state" in
        subcmd)
            _describe 'subcommand' subcommands
            ;;
        args)
            case $line[1] in
                start)
                    _arguments \
                        '--cwd[Set working directory]:directory:_files -/' \
                        '--workspace[Target workspace]:workspace:' \
                        '--tab[Target tab]:tab:' \
                        '--split[Split direction]:direction:(right down)' \
                        '--focus[Focus the agent]' \
                        '--no-focus[Do not focus the agent]' \
                        '1:name:' \
                        '*::argv:'
                    ;;
                read)
                    _arguments \
                        '--source[Output source]:source:(visible recent recent-unwrapped)' \
                        '--lines[Number of lines]:lines:' \
                        '--format[Output format]:format:(text ansi)' \
                        '1:agent:'
                    ;;
                rename)
                    _arguments \
                        '1:agent:' \
                        '2:label:'
                    ;;
                wait)
                    _arguments \
                        '--status[Wait for status]:status:(idle working blocked done unknown)' \
                        '--timeout[Timeout in milliseconds]:timeout:' \
                        '1:agent:'
                    ;;
            esac
            ;;
    esac
}

_herdr_channel() {
    local -a subcommands
    subcommands=(
        'show:Show current update channel'
        'set:Set update channel'
    )

    _arguments -C \
        '1: :->subcmd' \
        '*::arg:->args'

    case "$state" in
        subcmd)
            _describe 'subcommand' subcommands
            ;;
        args)
            case $line[1] in
                set)
                    _arguments '1:channel:(stable preview)'
                    ;;
            esac
            ;;
    esac
}

_herdr_completion() {
    local -a shells
    shells=(
        'zsh:Generate zsh completion script'
    )
    _describe 'shell' shells
}

_herdr_config() {
    local -a subcommands
    subcommands=(
        'reset-keys:Reset keybindings to defaults'
    )
    _describe 'subcommand' subcommands
}

_herdr_integration() {
    local -a subcommands
    subcommands=(
        'verify:Verify integration assets'
        'upgrade:Upgrade integration assets'
        'repair:Repair integration assets'
    )
    _describe 'subcommand' subcommands
}

_herdr_pane() {
    local -a subcommands
    subcommands=(
        'list:List all panes'
        'get:Get pane details'
        'read:Read pane output'
        'rename:Rename a pane'
        'split:Split a pane'
        'close:Close a pane'
        'send-text:Send text to pane'
        'send-keys:Send keys to pane'
        'report-agent:Report agent state'
        'report-metadata:Report metadata'
        'run:Run command in pane'
    )

    _arguments -C \
        '1: :->subcmd' \
        '*::arg:->args'

    case "$state" in
        subcmd)
            _describe 'subcommand' subcommands
            ;;
        args)
            case $line[1] in
                list)
                    _arguments '--workspace[Filter by workspace]:workspace:'
                    ;;
                read)
                    _arguments \
                        '--source[Output source]:source:(visible recent recent-unwrapped)' \
                        '--lines[Number of lines]:lines:' \
                        '--format[Output format]:format:(text ansi)' \
                        '1:pane_id:'
                    ;;
                rename)
                    _arguments \
                        '1:pane_id:' \
                        '2:label:'
                    ;;
                split)
                    _arguments \
                        '--cwd[Set working directory]:directory:_files -/' \
                        '--label[Pane label]:label:' \
                        '--split[Split direction]:direction:(right down)' \
                        '--focus[Focus the new pane]' \
                        '--no-focus[Do not focus the new pane]' \
                        '1:pane_id:' \
                        '*::argv:'
                    ;;
                report-agent)
                    _arguments \
                        '--status[Agent status]:status:(idle working blocked done unknown)' \
                        '1:pane_id:'
                    ;;
            esac
            ;;
    esac
}

_herdr_server() {
    local -a subcommands
    subcommands=(
        'stop:Stop the running server'
        'reload-config:Reload configuration'
        'live-handoff:Perform live handoff'
    )

    _arguments -C \
        '1: :->subcmd' \
        '*::arg:->args'

    case "$state" in
        subcmd)
            _describe 'subcommand' subcommands
            ;;
        args)
            case $line[1] in
                live-handoff)
                    _arguments \
                        '--import-exe[Import executable path]:path:_files' \
                        '--expected-protocol[Expected protocol version]:version:' \
                        '--expected-version[Expected version]:version:'
                    ;;
            esac
            ;;
    esac
}

_herdr_session() {
    local -a subcommands
    subcommands=(
        'list:List all sessions'
        'attach:Attach to a session'
        'stop:Stop a session'
        'delete:Delete a session'
    )

    _arguments -C \
        '1: :->subcmd' \
        '*::arg:->args'

    case "$state" in
        subcmd)
            _describe 'subcommand' subcommands
            ;;
        args)
            case $line[1] in
                list|stop|delete)
                    _arguments '--json[Output as JSON]'
                    ;;
                attach|stop|delete)
                    _arguments '1:name:'
                    ;;
            esac
            ;;
    esac
}

_herdr_status() {
    local -a subcommands
    subcommands=(
        'server:Show server status'
        'client:Show client status'
    )
    _describe 'subcommand' subcommands
}

_herdr_tab() {
    local -a subcommands
    subcommands=(
        'list:List all tabs'
        'get:Get tab details'
        'rename:Rename a tab'
        'close:Close a tab'
    )

    _arguments -C \
        '1: :->subcmd' \
        '*::arg:->args'

    case "$state" in
        subcmd)
            _describe 'subcommand' subcommands
            ;;
        args)
            case $line[1] in
                list)
                    _arguments '--workspace[Filter by workspace]:workspace:'
                    ;;
                rename)
                    _arguments \
                        '1:tab_id:' \
                        '2:label:'
                    ;;
                get|close)
                    _arguments '1:tab_id:'
                    ;;
            esac
            ;;
    esac
}

_herdr_wait() {
    local -a subcommands
    subcommands=(
        'output:Wait for text in output'
        'agent-status:Wait for agent status'
    )

    _arguments -C \
        '1: :->subcmd' \
        '*::arg:->args'

    case "$state" in
        subcmd)
            _describe 'subcommand' subcommands
            ;;
        args)
            case $line[1] in
                output)
                    _arguments \
                        '--match[Text to match]:text:' \
                        '--source[Output source]:source:(visible recent recent-unwrapped)' \
                        '--lines[Number of lines]:lines:' \
                        '--timeout[Timeout in milliseconds]:timeout:' \
                        '--regex[Use regex matching]' \
                        '--raw[Raw output]' \
                        '1:pane_id:'
                    ;;
                agent-status)
                    _arguments \
                        '--status[Status to wait for]:status:(idle working blocked done unknown)' \
                        '--timeout[Timeout in milliseconds]:timeout:' \
                        '1:pane_id:'
                    ;;
            esac
            ;;
    esac
}

_herdr_workspace() {
    local -a subcommands
    subcommands=(
        'list:List all workspaces'
        'create:Create a new workspace'
        'get:Get workspace details'
        'focus:Focus a workspace'
        'rename:Rename a workspace'
        'close:Close a workspace'
    )

    _arguments -C \
        '1: :->subcmd' \
        '*::arg:->args'

    case "$state" in
        subcmd)
            _describe 'subcommand' subcommands
            ;;
        args)
            case $line[1] in
                create)
                    _arguments \
                        '--cwd[Set working directory]:directory:_files -/' \
                        '--label[Workspace label]:label:' \
                        '--focus[Focus the workspace]' \
                        '--no-focus[Do not focus the workspace]'
                    ;;
                rename)
                    _arguments \
                        '1:workspace_id:' \
                        '2:label:'
                    ;;
                get|focus|close)
                    _arguments '1:workspace_id:'
                    ;;
            esac
            ;;
    esac
}

_herdr_worktree() {
    local -a subcommands
    subcommands=(
        'list:List all worktrees'
        'add:Add a new worktree'
        'remove:Remove a worktree'
    )

    _arguments -C \
        '1: :->subcmd' \
        '*::arg:->args'

    case "$state" in
        subcmd)
            _describe 'subcommand' subcommands
            ;;
        args)
            case $line[1] in
                add)
                    _arguments \
                        '--cwd[Set working directory]:directory:_files -/' \
                        '1:path:_files -/' \
                        '2:branch:'
                    ;;
                remove)
                    _arguments '1:path:_files -/'
                    ;;
            esac
            ;;
    esac
}

_herdr "$@"
"#
}
