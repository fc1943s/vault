```빙
```vim
풍크치옹! 비주아우세아르시(지렉치옹)
function! VisualSearch(direction)
레트 르:헤지스테르=@@
let l:register=@@
노르마우! 그비
normal! gvy
레트 르:세아르시=이스카피(@@, '$.*/\[]')
let l:search=escape(@@, '$.*/\[]')
이프 아:지렉치옹=='/'
if a:direction=='/'
이제쿠치 '노르마우! /'.르:세아르시
execute 'normal! /'.l:search
에우시
else
이제쿠치 '노르마우! ?'.르:세아르시
execute 'normal! ?'.l:search
인지프
endif
레트 @/=르:세아르시
let @/=l:search
노르마우! 그브
normal! gV
레트 @@=르:헤지스테르
let @@=l:register
인드풍크치옹
endfunction

세트 라지레드라
set lazyredraw
세트 비주아우베우
set visualbell
세트 노에호르베우스
set noerrorbells

레트 마플레아데르=" "
let mapleader=" "
레트 @아 = "테스트"
let @a = "text"

노레마프 <크-이> 7<크-이>
noremap <C-e> 7<C-e>
노레마프 <레아데르>이 7<크-이>
noremap <leader>e 7<C-e>

노레마프 <크-이> 7<크-이>
noremap <C-y> 7<C-y>
노레마프 <레아데르>이 7<크-이>
noremap <leader>y 7<C-y>

노레마프 <레아데르>프 <크-프>
noremap <leader>f <C-f>
노레마프 <레아데르>브 <크-브>
noremap <leader>b <C-b>

노레마프 <레아데르>사 <큼드>카우 브스코데노치피('이지토르.악치옹.셀렉타우')<크르>
noremap <leader>sa <Cmd>call VSCodeNotify('editor.action.selectAll')<CR>
노레마프 <레아데르>구 <큼드>카우 브스코데노치피('워르크벤시.악치옹.고톨리니')<크르>
noremap <leader>go <Cmd>call VSCodeNotify('workbench.action.gotoLine')<CR>

노레마프 <레아데르>응 응
noremap <leader>m M
노레마프 <레아데르><레아데르>4 $
noremap <leader><leader>4 $
노레마프 <레아데르>트 :악치옹 파라메테린푸<크르>
noremap <leader>tt :action ParameterInfo<CR>
노레마프 <레아데르>치 :악치옹 소웨호르데스크리프치옹<크르>
noremap <leader>te :action ShowErrorDescription<CR>

노레마프 <레아데르>즈스 우```즈스<인테르>```<이스크>우
noremap <leader>js o```js<enter>```<esc>O
노레마프 <레아데르>프스 우```프스<인테르>```<이스크>우
noremap <leader>fs o```fs<enter>```<esc>O


노레마프 <레아데르>브 브
noremap <leader>vv V

노레마프 <레아데르>이 이
noremap <leader>y y
노레마프 <레아데르>이 이
noremap <leader>Y Y
브노레마프 드 즈
vnoremap d j
브노레마프 <레아데르>드 드
vnoremap <leader>d d

노레마프 <레아데르>1 "지
noremap <leader>1 "zy
노레마프 <레아데르>2 "지
noremap <leader>2 "zY
노레마프 <레아데르>3 "스프
noremap <leader>3 "zp
노레마프 <레아데르><레아데르>3 "스프
noremap <leader><leader>3 "zP
노레마프 <레아데르>4 "즈드
noremap <leader>4 "zd

노레마프 <레아데르><레아데르>' 이````<이스크>이
noremap <leader><leader>' i````<Esc>hi
이노레마프 <레아데르><레아데르>' ````<이스크>이
inoremap <leader><leader>' ````<Esc>hi


노레마프 <레아데르>[ ?레트 <크르>:노<크르>
nnoremap <leader>[ ?let <CR>:noh<CR>
노레마프 <레아데르>] /레트 <크르>:노<크르>
nnoremap <leader>] /let <CR>:noh<CR>

노레마프 <레아데르>흐흐 :악치옹 헤룽<크르>
noremap <leader>rr :action Rerun<CR>
노레마프 <레아데르>후 :악치옹 훙<크르>
noremap <leader>ru :action Run<CR>
노레마프 <레아데르>흐스 :악치옹 스토프<크르>
noremap <leader>rs :action Stop<CR>
노레마프 <레아데르>흐크 :악치옹 훙콘피구라치옹<크르>
noremap <leader>rc :action RunConfiguration<CR>

노레마프 <레아데르>킁 :악치옹 클로제아우노치피카치옹스<크르>
noremap <leader>cn :action CloseAllNotifications<CR>
노레마프 <레아데르>쿠 :악치옹 클로제알레지토르즈부탁치비<크르>
noremap <leader>co :action CloseAllEditorsButActive<CR>

노레마프 <레아데르>하 <큼드>카우 브스코데노치피('노테보크.이제쿠치')<크르>
noremap <leader>ra <Cmd>call VSCodeNotify('notebook.execute')<CR>
노레마프 <레아데르>히 :헤두<크르>
noremap <leader>re :redo<CR>
노레마프 <레아데르>니 <큼드>카우 브스코데노치피('이지토르.악치옹.마르케르.네스트')<크르>
noremap <leader>ne <Cmd>call VSCodeNotify('editor.action.marker.next')<CR>
노레마프 <레아데르>피 <큼드>카우 브스코데노치피('이지토르.악치옹.마르케르.프레브')<크르>
noremap <leader>pe <Cmd>call VSCodeNotify('editor.action.marker.prev')<CR>

노레마프 <레아데르>응크 <큼드>카우 브스코데노치피('노테보크.포쿠즈네스테지토르')<크르>
noremap <leader>nc <Cmd>call VSCodeNotify('notebook.focusNextEditor')<CR>
노레마프 <레아데르>프크 <큼드>카우 브스코데노치피('노테보크.포쿠스프레비오제지토르')<크르>
noremap <leader>pc <Cmd>call VSCodeNotify('notebook.focusPreviousEditor')<CR>

노레마프 <레아데르>누 :악치옹 네스토쿠렌시<크르>
noremap <leader>no :action NextOccurence<CR>
노레마프 <레아데르>푸 :악치옹 프레비오조쿠렌시<크르>
noremap <leader>po :action PreviousOccurence<CR>

노레마프 크 /
noremap Q /
노레마프 <스파시>크 ?
noremap <space>Q ?
노레마프 / 즈
noremap / j
노레마프 ? 즈
noremap ? j


이프 인브 ==? "스"
if env ==? "sh"

노레마프 <크-히그트> 그트
nnoremap <c-right> gt
노레마프 <크-레프트> 그트
nnoremap <c-left> gT

노레마프 <실렌트><이스크><이스크> :노<크르>
nnoremap <silent><esc><esc> :noh<CR>

노레마프 <레아데르> /[아-스]<크르>:노<크르>
nnoremap <leader>w /[A-Z]<CR>h:noh<CR>
노레마프 <레아데르> /[아-스]<크르>:노<크르>
nnoremap <leader>W /[A-Z]<CR>:noh<CR>

브노레마프 <레아데르> /[아-스]<크르>
vnoremap <leader>w /[A-Z]<CR>h
브노레마프 <레아데르> /[아-스]<크르>
vnoremap <leader>W /[A-Z]<CR>

브노레마프 <실렌트>* <이스크>:카우 비주아우세아르시('/')<크르>/<크르>
vnoremap <silent>* <ESC>:call VisualSearch('/')<CR>/<CR>
브노레마프 <실렌트># <이스크>:카우 비주아우세아르시('?')<크르>?<크르>
vnoremap <silent># <ESC>:call VisualSearch('?')<CR>?<CR>

이우세이프 인브 ==? "이데아"
elseif env ==? "idea"

노레마프 <레아데르> ]
noremap <leader>w ]w
노레마프 <레아데르> [
noremap <leader>W [w

세트 이그노레카지
set ignorecase

인지프
endif

세트 이그노레카지
set ignorecase
노레마프 <레아데르> /\크[아-스_]<크르>:노<크르>
nnoremap <leader>w /\C[A-Z_]<CR>h:noh<CR>
노레마프 <레아데르> /\크[아-스_]<크르>:노<크르>
nnoremap <leader>W /\C[A-Z_]<CR>:noh<CR>
브노레마프 <레아데르> /\크[아-스_]<크르>
vnoremap <leader>w /\C[A-Z_]<CR>h
브노레마프 <레아데르> /\크[아-스_]<크르>
vnoremap <leader>W /\C[A-Z_]<CR>


노레마프 <크-[> <노프>
nnoremap <c-[> <nop>
노레마프 <크-]> <노프>
nnoremap <c-]> <nop>
노레마프 <크-\> <노프>
nnoremap <c-\> <nop>
노레마프 <크-스-[> <노프>
nnoremap <c-s-[> <nop>
노레마프 <크-스-]> <노프>
nnoremap <c-s-]> <nop>
노레마프 <크-스-\> <노프>
nnoremap <c-s-\> <nop>

노레마프 <노프>
nnoremap ; <nop>
노레마프 <크-> <노프>
nnoremap <c-;> <nop>
노레마프 <크-스-> <노프>
nnoremap <c-s-;> <nop>

노레마프 <레아데르>< <노프>
nnoremap <leader>< <nop>
노레마프 <레아데르>> <노프>
nnoremap <leader>> <nop>
노레마프 <레아데르>- <노프>
nnoremap <leader>- <nop>
노레마프 <레아데르>= <노프>
nnoremap <leader>= <nop>


노레마프 ! <노프>
nnoremap ! <nop>
노레마프 <스-타브> <노프>
nnoremap <s-tab> <nop>
노레마프 <스-프1> <노프>
nnoremap <s-f1> <nop>
노레마프 <스-프5> <노프>
nnoremap <s-f5> <nop>
노레마프 <크-크> <노프>
nnoremap <c-k> <nop>
노레마프 ,드 <노프>
nnoremap ,d <nop>
노레마프 <레아데르><레아데르> <노프>
nnoremap <leader><leader> <nop>
노레마프 <레아데르>즈 <노프>
nnoremap <leader>j <nop>
노레마프 <레아데르>즈 <노프>
nnoremap <leader>J <nop>
"" 노레마프 <타브> <노프>
"" nnoremap <tab> <nop>

"" 노레마프 <레아데르><레아데르>/ 이／
"" noremap <leader><leader>/ i／
"" 이노레마프 <레아데르><레아데르>/ ／
"" inoremap <leader><leader>/ ／
"" 노레마프 이 즈
"" noremap y j
"" 노레마프 이 즈
"" noremap Y j

"" 노레마프 <레아데르>구 올레트 제틀로카우스 () = $"0={0} {제틀로카우스 ()}"<이스크>0즈
"" noremap <leader>gl Olet getLocals () = $"0={0} {getLocals ()}"<Esc>0jw
"" 노레마프 <레아데르>능 그세나메스파시<이스크>이비"므슈<이스크>오모둘리 <이스크>"므파 =<이스크>우()<이스크>>>
"" noremap <leader>nm ggcenamespace<esc>Eviw"mxxo<esc>omodule <esc>"mpa =<esc>o()<esc>>>

```
```
