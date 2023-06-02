-- A Lua script to extract a theme from Neovim for syntastica.
-- Run with `nvim --headless -c 'colorscheme <theme_name>' -c 'lua require("nvim_extract")' +qa 2>&1 | sed 's/\r//g'`

local function parse_color(col)
    local hex = string.format('%06x', col)
    local red = tonumber(hex:sub(1, 2), 16)
    local green = tonumber(hex:sub(3, 4), 16)
    local blue = tonumber(hex:sub(5, 6), 16)
    return red, green, blue
end

local all_highlights = vim.api.nvim_get_hl(0, {})

print('    ResolvedTheme::new(BTreeMap::from([')

-- set the `bg0` key to the themes background color (if present)
local normal = vim.api.nvim_get_hl(0, { name = 'Normal' })
if normal.bg ~= nil then
    local line = '        ("bg0".to_owned(), Style::color_only('
    local r, g, b = parse_color(normal.bg)
    line = line .. r .. ', ' .. g .. ', ' .. b .. ')),'
    print(line)
end

for key, _ in pairs(all_highlights) do
    -- skip all highlights not starting with `@` or starting with `@lsp`
    if key:sub(1, 1) ~= '@' or key:find('@lsp', 1, true) == 1 then
        goto continue
    end

    -- follow all links
    local style = vim.api.nvim_get_hl(0, { name = key })
    while style.link ~= nil do
        style = vim.api.nvim_get_hl(0, { name = style.link })
    end

    -- skip this highlight if no color is set
    if style.fg == nil then
        goto continue
    end

    local value = 'Style::new('
    local r, g, b = parse_color(style.fg)
    value = value .. 'Color::new(' .. r .. ', ' .. g .. ', ' .. b .. '), '
    value = value .. tostring(style.underline or false) .. ', '
    value = value .. tostring(style.strikethrough or false) .. ', '
    value = value .. tostring(style.italic or false) .. ', '
    value = value .. tostring(style.bold or false) .. ')'
    print('        ("' .. key:sub(2) .. '".to_owned(), ' .. value .. '),')

    ::continue::
end
print('    ]))')
