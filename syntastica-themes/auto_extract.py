#!/usr/bin/env python

import subprocess

class SubTheme:
    """
    A class to represent a sub-theme of a colorscheme.

    Attributes
    ----------
    name : str
        The name of the sub-theme.
    activate_command : str
        The command to activate the sub-theme in nvim.

    Methods
    -------
    __repr__()
        Returns a string representation of the sub-theme.
    get_from_nvim()
        Returns a string containing the Rust code for the sub-theme extracted from nvim.
    to_rust()
        Returns a string containing the Rust function definition for the sub-theme.
    """

    def __init__(self, name: str, activate_command: str):
        """
        Constructs all the necessary attributes for the sub-theme object.

        Parameters
        ----------
        name : str
            The name of the sub-theme.
        activate_command : str
            The command to activate the sub-theme in nvim.
        """

        self.name = name
        self.activate_command = activate_command

    def __repr__(self) -> str:
        """
        Returns a string representation of the sub-theme.

        Returns
        -------
        str
            A string containing the name and the activate command of the sub-theme.
        """

        return f"{self.name}: `{self.activate_command}`"

    def get_from_nvim(self) -> str:
        """
        Returns a string containing the Rust code for the sub-theme extracted from nvim.

        Raises
        ------
        Exception
            If there is an error in running the system command or extracting the colorscheme from nvim.

        Returns
        -------
        str
            A string containing the Rust code for the sub-theme.
        """

        system_command = ["nvim", "--headless", "-c", self.activate_command, "-c", "lua require('nvim_extract')", "+qa"]
        output = subprocess.run(system_command, capture_output=True, text=True).stderr
        if "Error" in output:
            raise Exception(f"Could not get colorscheme '{self}' from nvim, make sure you have nvim with the theme installed and are in the same directory as the lua script")
        return output

    def to_rust(self) -> str:
        """
        Returns a string containing the Rust function definition for the sub-theme.

        Returns
        -------
        str
            A string containing the Rust function definition for the sub-theme.
        """

        return f"""
#[rustfmt::skip]
pub fn {self.name}() -> ResolvedTheme {{
{self.get_from_nvim()}
}}
"""

class Theme:
    """
    A class to represent a theme consisting of multiple sub-themes.

    Attributes
    ----------
    name : str
        The name of the theme.
    url : str
        The URL of the theme's source repository.
    sub_themes : [SubTheme]
        A list of SubTheme objects representing the sub-themes of the theme.

    Methods
    -------
    __repr__()
        Returns a string representation of the theme.
    to_rust()
        Returns a string containing the Rust module definition for the theme.
    """

    def __init__(self, name: str, url: str, sub_themes: [SubTheme]):
        """
        Constructs all the necessary attributes for the theme object.

        Parameters
        ----------
        name : str
            The name of the theme.
        url : str
            The URL of the theme's source repository.
        sub_themes : [SubTheme]
            A list of SubTheme objects representing the sub-themes of the theme.
        """

        self.name = name
        self.url = url
        self.sub_themes = sub_themes

    def __repr__(self) -> str:
        """
        Returns a string representation of the theme.

        Returns
        -------
        str
            A string containing the name, URL and sub-themes of the theme.
        """

        return f"{self.name} ({self.url}): {self.sub_themes}"

    def to_rust(self) -> str:
        """
        Returns a string containing the Rust module definition for the theme.

        Returns
        -------
        str
            A string containing the Rust module definition for the theme.
        """

        rust = f"""//! The {self.name} theme collection in this module was extracted from <{self.url}> using `auto_extract.py`.

use std::collections::BTreeMap;

use syntastica_core::{{
    style::{{Color, Style}},
    theme::ResolvedTheme,
}};
"""

        for sub_theme in self.sub_themes:
            rust += sub_theme.to_rust()

        return rust


def import_themes(themes: [Theme]):
    """
    Imports a list of themes into the Rust codebase.

    This function iterates over the themes and writes their Rust module definitions to a file named `theme_name.rs` in the `src` directory.

    Parameters
    ----------
    themes : [Theme]
        A list of Theme objects representing the themes to be imported.

    Raises
    ------
    Exception
        If there is an error in writing to the file or importing the themes.
    """

    for theme in themes:
        filename = f"src/{theme.name}.rs"
        print(f"writing {filename} ..")
        with open(filename, "w") as f:
            f.write(theme.to_rust())
        print(".. done")


if __name__ == "__main__":
    """
    Import all the themes specified to syntastica, has to be run from its
    location, nvim has to be installed with the themes to import installed
    """

    # List of packer.nvim packages of the themes below
    # use('folke/tokyonight.nvim')
    # use { 'catppuccin/nvim', as = 'catppuccin' }
    # use('projekt0n/github-nvim-theme')
    # use('shaunsingh/nord.nvim')
    # use('Mofiqul/vscode.nvim')
    # use('dracula/vim')
    # use('overcache/NeoSolarized')
    # use('tanvirtin/monokai.nvim')
    # use('doums/darcula')
    # use('Abstract-IDE/Abstract-cs')
    # use('rafamadriz/neon')
    # use('marko-cerovac/material.nvim')
    # use('bluz71/vim-nightfly-colors')
    # use('bluz71/vim-moonfly-colors')
    # use('nyoom-engineering/oxocarbon.nvim')
    # use('kyazdani42/blue-moon')
    # use('mhartington/oceanic-next')
    # use('glepnir/zephyr-nvim')
    # use('rockerBOO/boo-colorscheme-nvim')
    # use('yonlu/omni.vim')
    # use('ray-x/aurora')
    # use('ray-x/starry.nvim')
    # use('shaunsingh/moonlight.nvim')
    # use('fenetikm/falcon')
    # use('savq/melange-nvim')
    # use('sainnhe/everforest')
    # use('kdheepak/monochrome.nvim')
    # use('Everblush/nvim')
    # use('Yazeed1s/minimal.nvim')

    themes_to_import = [
        Theme("catppuccin", "https://github.com/catppuccin/nvim", [
            SubTheme(
                "latte",
                "colorscheme catppuccin-latte"
                ),
            SubTheme(
                "mocha",
                "colorscheme catppuccin-mocha"
                ),
            SubTheme(
                "frappe",
                "colorscheme catppuccin-frappe"
                ),
            SubTheme(
                "macchiato",
                "colorscheme catppuccin-macchiato"
                ),
            ]
        ),
        Theme("tokyo", "https://github.com/folke/tokyonight.nvim", [
            SubTheme(
                "storm",
                "colorscheme tokyonight-storm"
                ),
            SubTheme(
                "night",
                "colorscheme tokyonight-night"
                ),
            SubTheme(
                "day",
                "colorscheme tokyonight-day"
                ),
            SubTheme(
                "moon",
                "colorscheme tokyonight-moon"
                ),
            ]
        ),
        Theme("github", "https://github.com/project0n/github-nvim-theme", [
            SubTheme(
                "dark",
                "colorscheme github_dark"
                ),
            SubTheme(
                "dark_colorblind",
                "colorscheme github_dark_colorblind"
                ),
            SubTheme(
                "light",
                "colorscheme github_light"
                ),
            SubTheme(
                "dimmed",
                "colorscheme github_dimmed"
                ),
            SubTheme(
                "dark_dimmed",
                "colorscheme github_dark_dimmed"
                ),
            SubTheme(
                "dark_default",
                "colorscheme github_dark_default"
                ),
            SubTheme(
                "light_default",
                "colorscheme github_light_default"
                ),
            SubTheme(
                "dark_tritanopia",
                "colorscheme github_dark_tritanopia"
                ),
            SubTheme(
                "light_colorblind",
                "colorscheme github_light_colorblind"
                ),
            SubTheme(
                "light_tritanopia",
                "colorscheme github_light_tritanopia"
                ),
            SubTheme(
                "dark_high_contrast",
                "colorscheme github_dark_high_contrast"
                ),
            SubTheme(
                "light_high_contrast",
                "colorscheme github_light_high_contrast"
                ),
            ]
        ),
        Theme("nord", "https://github.com/shaunsingh/nord.nvim", [
            SubTheme(
                "nord",
                "colorscheme nord"
                ),
            ]
        ),
        Theme("vscode", "https://github.com/Mofiqul/vscode.nvim", [
            SubTheme(
                "dark",
                "lua require('vscode').load('dark')"
                ),
            SubTheme(
                "light",
                "lua require('vscode').load('light')"
                ),
            ]
        ),
        Theme("dracula", "https://github.com/dracula/vim", [
            SubTheme(
                "dracula",
                "colorscheme dracula"
                ),
            ]
        ),
        Theme("solarized", "https://github.com/overcache/NeoSolarized", [
            SubTheme(
                "dark",
                "colorscheme NeoSolarized"
                ),
            SubTheme(
                "light",
                "set background=light | colorscheme NeoSolarized"
                ),
            ]
        ),
        Theme("monokai", "https://github.com/tanvirtin/monokai.nvim", [
            SubTheme(
                "monokai",
                "colorscheme monokai"
                ),
            SubTheme(
                "ristretto",
                "colorscheme monokai_ristretto"
                ),
            SubTheme(
                "soda",
                "colorscheme monokai_soda"
                ),
            SubTheme(
                "pro",
                "colorscheme monokai_pro"
                ),
            ]
        ),
        Theme("dracula", "https://github.com/doums/darcula", [
            SubTheme(
                "dracula",
                "colorscheme dracula"
                ),
            ]
        ),
        Theme("abscs", "https://github.com/Abstract-IDE/Abstract-cs", [
            SubTheme(
                "abscs",
                "colorscheme abscs"
                ),
            ]
        ),
        Theme("neon", "https://github.com/rafamadriz/neon", [
            SubTheme(
                "default",
                "colorscheme neon"
                ),
            SubTheme(
                "doom",
                "let g:neon_style='doom' | colorscheme neon"
                ),
            SubTheme(
                "dark",
                "let g:neon_style='dark' | colorscheme neon"
                ),
            SubTheme(
                "light",
                "let g:neon_style='light' | colorscheme neon"
                ),
            ]
        ),
        Theme("material", "https://github.com/marko-cerovac/material.nvim", [
            SubTheme(
                "darker",
                "let g:material_style='darker' | colorscheme material"
                ),
            SubTheme(
                "lighter",
                "let g:material_style='lighter' | colorscheme material"
                ),
            SubTheme(
                "oceanic",
                "let g:material_style='oceanic' | colorscheme material"
                ),
            SubTheme(
                "palenight",
                "let g:material_style='palenight' | colorscheme material"
                ),
            SubTheme(
                "deep_ocean",
                "let g:material_style='deep ocean' | colorscheme material"
                ),
            ]
        ),
        Theme("nightfly", "https://github.com/bluz71/vim-nightfly-colors", [
            SubTheme(
                "nightfly",
                "colorscheme nightfly"
                ),
            ]
        ),
        Theme("moonfly", "https://github.com/bluz71/vim-moonfly-colors", [
            SubTheme(
                "moonfly",
                "colorscheme moonfly"
                ),
            ]
        ),
        Theme("oxocarbon", "https://github.com/nyoom-engineering/oxocarbon.nvim", [
            SubTheme(
                "dark",
                "colorscheme oxocarbon"
                ),
            SubTheme(
                "light",
                "set background=light | colorscheme oxocarbon"
                ),
            ]
        ),
        Theme("blue_moon", "https://github.com/kyazdani42/blue-moon", [
            SubTheme(
                "blue_moon",
                "colorscheme blue-moon"
                ),
            ]
        ),
        Theme("oceanicnext", "https://github.com/mhartington/oceanic-next", [
            SubTheme(
                "dark",
                "colorscheme OceanicNext"
                ),
            SubTheme(
                "light",
                "colorscheme OceanicNextLight"
                ),
            ]
        ),
        Theme("zephyr", "https://github.com/glepnir/zephyr-nvim", [
            SubTheme(
                "zephyr",
                "colorscheme zephyr"
                ),
            ]
        ),
        Theme("boo", "https://github.com/rockerBOO/boo-colorscheme-nvim", [
            SubTheme(
                "boo",
                "colorscheme boo"
                ),
            ]
        ),
        Theme("omni", "https://github.com/yonlu/omni", [
            SubTheme(
                "omni",
                "colorscheme omni"
                ),
            ]
        ),
        Theme("aurora", "https://github.com/ray-x/aurora", [
            SubTheme(
                "aurora",
                "colorscheme aurora"
                ),
            ]
        ),
        Theme("starry", "https://github.com/ray-x/starrt", [
            SubTheme(
                "starry",
                "colorscheme starry"
                ),
            ]
        ),
        Theme("moonlight", "https://github.com/shaunsingh/moonlight.nvim", [
            SubTheme(
                "moonlight",
                "colorscheme moonlight"
                ),
            ]
        ),
        Theme("falcon", "https://github.com/fenetikm/falcon", [
            SubTheme(
                "falcon",
                "colorscheme falcon"
                ),
            ]
        ),
        Theme("melange", "https://github.com/savq/melange-nvim", [
            SubTheme(
                "melange",
                "colorscheme melange"
                ),
            ]
        ),
        Theme("everforest", "https://github.com/sainnhe/everforest", [
            SubTheme(
                "dark",
                "set background=dark | colorscheme melange"
                ),
            SubTheme(
                "light",
                "set background=light | colorscheme melange"
                ),
            ]
        ),
        Theme("monochrome", "https://github.com/kdheepak/monochrome.nvim", [
            SubTheme(
                "monochrome",
                "colorscheme monochrome"
                ),
            ]
        ),
        Theme("everblush", "https://github.com/Everblush/nvim", [
            SubTheme(
                "everblush",
                "colorscheme everblush"
                ),
            ]
        ),
        Theme("minimal", "https://github.com/Yazeed1s/minimal.nvim", [
            SubTheme(
                "minimal",
                "colorscheme minimal"
                ),
            ]
        ),
    ]

    import_themes(themes_to_import)
