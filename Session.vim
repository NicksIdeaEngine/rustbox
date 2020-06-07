let SessionLoad = 1
let s:so_save = &so | let s:siso_save = &siso | set so=0 siso=0
let v:this_session=expand("<sfile>:p")
silent only
cd /mnt/sdc1/dbx/proj/rustbox
if expand('%') == '' && !&modified && line('$') <= 1 && getline(1) == ''
  let s:wipebuf = bufnr('%')
endif
set shortmess=aoO
badd +1 /mnt/sdc1/dbx/proj/rustbox
badd +1 hello.rs
badd +1 notes.md
badd +1 src/main.rs
badd +4 .gitignore
badd +1 Cargo.toml
badd +1 term://.//249860:/bin/zsh
badd +30 term://.//327294:/bin/zsh
argglobal
%argdel
$argadd /mnt/sdc1/dbx/proj/rustbox
edit src/main.rs
set splitbelow splitright
wincmd _ | wincmd |
vsplit
wincmd _ | wincmd |
vsplit
2wincmd h
wincmd w
wincmd w
wincmd t
set winminheight=0
set winheight=1
set winminwidth=0
set winwidth=1
exe 'vert 1resize ' . ((&columns * 85 + 128) / 256)
exe 'vert 2resize ' . ((&columns * 85 + 128) / 256)
exe 'vert 3resize ' . ((&columns * 84 + 128) / 256)
exe '4resize ' . ((&lines * 27 + 35) / 71)
exe 'vert 4resize ' . ((&columns * 85 + 128) / 256)
argglobal
setlocal fdm=marker
setlocal fde=0
setlocal fmr={{{,}}}
setlocal fdi=#
setlocal fdl=0
setlocal fml=1
setlocal fdn=10
setlocal fen
let s:l = 1 - ((0 * winheight(0) + 34) / 69)
if s:l < 1 | let s:l = 1 | endif
exe s:l
normal! zt
1
normal! 0
lcd /mnt/sdc1/dbx/proj/rustbox
wincmd w
argglobal
if bufexists("/mnt/sdc1/dbx/proj/rustbox/.gitignore") | buffer /mnt/sdc1/dbx/proj/rustbox/.gitignore | else | edit /mnt/sdc1/dbx/proj/rustbox/.gitignore | endif
setlocal fdm=marker
setlocal fde=0
setlocal fmr={{{,}}}
setlocal fdi=#
setlocal fdl=0
setlocal fml=1
setlocal fdn=10
setlocal fen
let s:l = 4 - ((3 * winheight(0) + 34) / 69)
if s:l < 1 | let s:l = 1 | endif
exe s:l
normal! zt
4
normal! 010|
lcd /mnt/sdc1/dbx/proj/rustbox
wincmd w
argglobal
if bufexists("/mnt/sdc1/dbx/proj/rustbox/Cargo.toml") | buffer /mnt/sdc1/dbx/proj/rustbox/Cargo.toml | else | edit /mnt/sdc1/dbx/proj/rustbox/Cargo.toml | endif
setlocal fdm=marker
setlocal fde=0
setlocal fmr={{{,}}}
setlocal fdi=#
setlocal fdl=0
setlocal fml=1
setlocal fdn=10
setlocal fen
let s:l = 5 - ((4 * winheight(0) + 34) / 69)
if s:l < 1 | let s:l = 1 | endif
exe s:l
normal! zt
5
normal! 0
lcd /mnt/sdc1/dbx/proj/rustbox
wincmd w
argglobal
if bufexists("term://.//327294:/bin/zsh") | buffer term://.//327294:/bin/zsh | else | edit term://.//327294:/bin/zsh | endif
setlocal fdm=marker
setlocal fde=0
setlocal fmr={{{,}}}
setlocal fdi=#
setlocal fdl=0
setlocal fml=1
setlocal fdn=10
setlocal fen
let s:l = 1 - ((0 * winheight(0) + 13) / 27)
if s:l < 1 | let s:l = 1 | endif
exe s:l
normal! zt
1
normal! 0
lcd /mnt/sdc1/dbx/proj/rustbox
wincmd w
4wincmd w
exe 'vert 1resize ' . ((&columns * 85 + 128) / 256)
exe 'vert 2resize ' . ((&columns * 85 + 128) / 256)
exe 'vert 3resize ' . ((&columns * 84 + 128) / 256)
exe '4resize ' . ((&lines * 27 + 35) / 71)
exe 'vert 4resize ' . ((&columns * 85 + 128) / 256)
tabnext 1
if exists('s:wipebuf') && getbufvar(s:wipebuf, '&buftype') isnot# 'terminal'
  silent exe 'bwipe ' . s:wipebuf
endif
unlet! s:wipebuf
set winheight=1 winwidth=20 winminheight=1 winminwidth=1 shortmess=filnxtToOFc
let s:sx = expand("<sfile>:p:r")."x.vim"
if file_readable(s:sx)
  exe "source " . fnameescape(s:sx)
endif
let &so = s:so_save | let &siso = s:siso_save
let g:this_session = v:this_session
let g:this_obsession = v:this_session
doautoall SessionLoadPost
unlet SessionLoad
" vim: set ft=vim :
