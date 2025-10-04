usage `./options_tree <path-to-jsonfile> <path-to-output-folder>`

this is more of "lern rust", cause i never done anyhting in rust before, so i was currios how it is.
Though i could combine lerning and something usefull for me at least

i am trying out nix and it difficult to nanigate in https://search.nixos.org/options cause i would love to have it in direcory tree structure, so every related together option would be in one place. I found out u can get all the options in jsonformat, but the pain is that its still the optoins chained together with a dot.
this little project was to create for every option an md file so it can be easily found and search in Obsidian, and yeah, works sofar.


to get the json files:

  - nixos configuration:
    here u have to navigate to your folder with your flake and use this command
    ```
    nix build .#nixosConfigurations.desktop.config.system.build.manual.optionsJSON
    ```
    the output folder will be in the same direcory

  - home manager:
    easy on that one
    set ```manual.json.enable = true``` in your home manager and the json file will be at `<profile directory>/share/doc/home-manager/options.json`
    or you can also use the same command as by nixos options but in your home-manager flake directory

i aint good in rust by any means, i just played around, to see what does what, so this is more of learn rust rather then usefull tool, still maybe for someone helpfull
