# wooden-puzzle-solver

This was an afternoon hack to generate all possible solutions to a
wooden puzzle, whose proper name is unknown.  Co-author:  Conor
McCutcheon.

The Python implementation came first.  Now there's a Rust
implementation too!

## Visualization

A browser-based visualization is available.  This uses the Rust
implementation, compiled to WebAssembly.  Run:

```
make && scripts/serve-site.sh
```

and then open `http://127.0.0.1:8000/` in your browser.

The visualization is also available as [a GitHub Pages site](https://jasonuhl.github.io/wooden-puzzle-solver/).

## License

This program is free software; you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation; either version 2 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License along
with this program; if not, write to the Free Software Foundation, Inc.,
51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.
