# README

## Development

### Tailwind Hack
There's a powerful dichotomy here:

* Dioxus won't work with a remote daisyUI plugin like `@plugin "daisyui"`
* The vscode tailwindcss extension won't work with a local daisyUI plugin like `@plugin "tailwind-plugins/daisyUI"`

Therefore, I just copypasted the damn `tailwind.css`, created one only for vscode like it likes it, and one for dioxus as it likes it. Told vscode to use the one it likes and likewise for dioxus. Real hacky but it works.
