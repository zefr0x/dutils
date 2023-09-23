function __dutils_autocomplete
  function subcommands
    set -l dutils_commands 'get help launch list open set unset'
    complete -f -c dutils -n "not __fish_seen_subcommand_from $dutils_commands" -a "get" -d "Show handler for mime"
    complete -f -c dutils -n "not __fish_seen_subcommand_from $dutils_commands" -a "launch" -d "Launch given handler with path/args"
    complete -f -c dutils -n "not __fish_seen_subcommand_from $dutils_commands" -a "list" -d "Show handlers (default applications)"
    complete -f -c dutils -n "not __fish_seen_subcommand_from $dutils_commands" -a "open" -d "Open path/URL with default handler (like xdg-open)"
    complete -f -c dutils -n "not __fish_seen_subcommand_from $dutils_commands" -a "set" -d "Set handler for extension (e.g. pdf) or mime type"
    complete -f -c dutils -n "not __fish_seen_subcommand_from $dutils_commands" -a "unset" -d "Unset handler"
  end

  function _set_add
    complete -f -c dutils -n '__fish_seen_subcommand_from set; __fish_prev_arg_in "set"' -a '(dutils autocomplete -m)'
    complete -f -c dutils -n '__fish_seen_subcommand_from set; set -l last (commandline -pco)[-2]; [ "$last" = "set" ]' -a '(dutils autocomplete -d)'

    complete -f -c dutils -n '__fish_seen_subcommand_from add; __fish_prev_arg_in "add"' -a '(dutils autocomplete -m)'
    complete -f -c dutils -n '__fish_seen_subcommand_from add; set -l last (commandline -pco)[-2]; [ "$last" = "add" ]' -a '(dutils autocomplete -d)'
  end

  subcommands
  _set_add
  complete -f -c dutils -n '__fish_seen_subcommand_from get' -a '(dutils autocomplete -m)'
  complete -f -c dutils -n '__fish_seen_subcommand_from get' -l 'json'
  complete -f -c dutils -n '__fish_seen_subcommand_from unset' -a '(dutils autocomplete -m)'
  complete -f -c dutils -n '__fish_seen_subcommand_from launch; __fish_prev_arg_in launch' -a '(dutils autocomplete -m)'

end

__dutils_autocomplete
