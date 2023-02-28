```vim
function! VisualSearch(direction)
    let l:register=@@
    normal! gvy
    let l:search=escape(@@, '$.*/\[]')
    if a:direction=='/'
        execute 'normal! /'.l:search
    else
        execute 'normal! ?'.l:search
    endif
    let @/=l:search
    normal! gV
    let @@=l:register
endfunction

set lazyredraw
set visualbell
set noerrorbells

let mapleader=" "
let @a = "text"

noremap <C-e> 7<C-e>
noremap <leader>e 7<C-e>

noremap <C-y> 7<C-y>
noremap <leader>y 7<C-y>

noremap <leader>f <C-f>
noremap <leader>b <C-b>

noremap <leader>sa <Cmd>call VSCodeNotify('editor.action.selectAll')<CR>
noremap <leader>go <Cmd>call VSCodeNotify('workbench.action.gotoLine')<CR>

noremap <leader>m M
noremap <leader><leader>4 $
noremap <leader>tt :action ParameterInfo<CR>
noremap <leader>te :action ShowErrorDescription<CR>

noremap <leader>js o```js<enter>```<esc>O
noremap <leader>fs o```fs<enter>```<esc>O


noremap <leader>vv V

noremap <leader>y y
noremap <leader>Y Y
vnoremap d j
vnoremap <leader>d d

noremap <leader>1 "zy
noremap <leader>2 "zY
noremap <leader>3 "zp
noremap <leader><leader>3 "zP
noremap <leader>4 "zd

noremap <leader><leader>' i````<Esc>hi
inoremap <leader><leader>' ````<Esc>hi


nnoremap <leader>[ ?let <CR>:noh<CR>
nnoremap <leader>] /let <CR>:noh<CR>

noremap <leader>rr :action Rerun<CR>
noremap <leader>ru :action Run<CR>
noremap <leader>rs :action Stop<CR>
noremap <leader>rc :action RunConfiguration<CR>

noremap <leader>cn :action CloseAllNotifications<CR>
noremap <leader>co :action CloseAllEditorsButActive<CR>

noremap <leader>ra <Cmd>call VSCodeNotify('notebook.execute')<CR>
noremap <leader>re :redo<CR>
noremap <leader>ne <Cmd>call VSCodeNotify('editor.action.marker.next')<CR>
noremap <leader>pe <Cmd>call VSCodeNotify('editor.action.marker.prev')<CR>

noremap <leader>nc <Cmd>call VSCodeNotify('notebook.focusNextEditor')<CR>
noremap <leader>pc <Cmd>call VSCodeNotify('notebook.focusPreviousEditor')<CR>

noremap <leader>no :action NextOccurence<CR>
noremap <leader>po :action PreviousOccurence<CR>

noremap Q /
noremap <space>Q ?
noremap / j
noremap ? j


if env ==? "sh"

    nnoremap <c-right> gt
    nnoremap <c-left> gT

    nnoremap <silent><esc><esc> :noh<CR>

    nnoremap <leader>w /[A-Z]<CR>h:noh<CR>
    nnoremap <leader>W /[A-Z]<CR>:noh<CR>

    vnoremap <leader>w /[A-Z]<CR>h
    vnoremap <leader>W /[A-Z]<CR>

    vnoremap <silent>* <ESC>:call VisualSearch('/')<CR>/<CR>
    vnoremap <silent># <ESC>:call VisualSearch('?')<CR>?<CR>

elseif env ==? "idea"

    noremap <leader>w ]w
    noremap <leader>W [w

    set ignorecase

endif

set ignorecase
nnoremap <leader>w /\C[A-Z_]<CR>h:noh<CR>
nnoremap <leader>W /\C[A-Z_]<CR>:noh<CR>
vnoremap <leader>w /\C[A-Z_]<CR>h
vnoremap <leader>W /\C[A-Z_]<CR>


nnoremap <c-[> <nop>
nnoremap <c-]> <nop>
nnoremap <c-\> <nop>
nnoremap <c-s-[> <nop>
nnoremap <c-s-]> <nop>
nnoremap <c-s-\> <nop>

nnoremap ; <nop>
nnoremap <c-;> <nop>
nnoremap <c-s-;> <nop>

nnoremap <leader>< <nop>
nnoremap <leader>> <nop>
nnoremap <leader>- <nop>
nnoremap <leader>= <nop>


nnoremap ! <nop>
nnoremap <s-tab> <nop>
nnoremap <s-f1> <nop>
nnoremap <s-f5> <nop>
nnoremap <c-k> <nop>
nnoremap ,d <nop>
nnoremap <leader><leader> <nop>
nnoremap <leader>j <nop>
nnoremap <leader>J <nop>
"" nnoremap <tab> <nop>

"" noremap <leader><leader>/ i／
"" inoremap <leader><leader>/ ／
"" noremap y j
"" noremap Y j

"" noremap <leader>gl Olet getLocals () = $"0={0} {getLocals ()}"<Esc>0jw
"" noremap <leader>nm ggcenamespace<esc>Eviw"mxxo<esc>omodule <esc>"mpa =<esc>o()<esc>>>

```
