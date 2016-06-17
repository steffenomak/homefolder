let g:ghcmod_ghc_options = ['-Wall']

au FileType haskell nnoremap <buffer> <F1> :GhcModType<CR>
au FileType haskell nnoremap <buffer> <F2> :GhcModTypeClear<CR>
au FileType haskell nnoremap <buffer> <F3> :GhcModCheckAsync<CR>
au FileType haskell nnoremap <buffer> <F4> :GhcModLintAsync<CR>

autocmd BufWritePost *.hs GhcModCheckAsync
