{ config, lib, pkgs, ... }:

with lib;

let
  cfg = config.bannify;
in {
  options = {
    bannify = {
      languages = mkOption {
        type = with types; listOf (submodule {
          options = {
            name = mkOption {
              type = types.str;
              description = "Language name";
              example = "Rust";
            };
            singleLineComment = mkOption {
              type = types.str;
              description = "Single line comment marker";
              example = "//";
            };
            extensions = mkOption {
              type = types.listOf types.str;
              description = "File extensions for the language";
              example = [ "rs" ];
            };
          };
        });
        default = [
          {
            name = "Rust";
            singleLineComment = "//";
            extensions = [ "rs" ];
          }
          {
            name = "Python";
            singleLineComment = "#";
            extensions = [ "py" ];
          }
          {
            name = "JavaScript";
            singleLineComment = "//";
            extensions = [ "js" "jsx" ];
          }
          {
            name = "C";
            singleLineComment = "//";
            extensions = [ "c" "h" ];
          }
          {
            name = "Markdown";
            singleLineComment = "<!--";
            extensions = [ "md" ];
          }
          {
            name = "HTML";
            singleLineComment = "<!--";
            extensions = [ "html" ];
          }
        ];
        description = "List of supported languages for Bannify.";
      };
    };
  };

  config = mkIf (cfg.languages != []) {
    systemd.user.services.bannify-config = {
      description = "Generate Bannify config";
      serviceConfig = {
        ExecStart = pkgs.writeShellScript "update-bannify-config" ''
          mkdir -p $HOME/.config
          config_file="$HOME/.config/bannify_config.toml"

          # If the config file doesn't exist, create it
          if [ ! -f "$config_file" ]; then
              touch "$config_file"
          fi

          echo "" > "$config_file"

          ${concatMapStringsSep "\n" (lang: ''
            echo '[[languages]]' >> "$config_file"
            echo "name = \"${lang.name}\"" >> "$config_file"
            echo "single_line_comment = \"${lang.singleLineComment}\"" >> "$config_file"
            echo "extensions = [${concatStringsSep ", " (map (e: ''"${e}"'') lang.extensions)}]" >> "$config_file"
          '') cfg.languages}
        '';
        Restart = "always";
      };

      wantedBy = [ "default.target" ];
    };
  };
}