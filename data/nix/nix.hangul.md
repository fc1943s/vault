
클레아르 && 니슈스-헤부이우드 스위트시 --우프그라지-아우
clear && nixos-rebuild switch --upgrade-all

스란드르 --세트프로비데로트푸트소르시 느비지아-그0 모데제칭그 && 스란드르 --아우투
xrandr --setprovideroutputsource NVIDIA-G0 modesetting && xrandr --auto
제니트
zenith
라프시 & 지종
lapce & disown

레드제르-리비-데스크토프 --누-산드보스 & 지종
ledger-live-desktop --no-sandbox & disown
수두 - -우 알리아스 레드제르-리비-데스크토프
sudo -H -u alias ledger-live-desktop

미크로조프트-에드지-데브 --누-산드보스 & 지종
microsoft-edge-dev --no-sandbox & disown
수두 - -우 알리아스 미크로조프트-에드지-데브
sudo -H -u alias microsoft-edge-dev
미크로조프트-에드지-데브 --지자블리-웨브-세쿠리치 --이그노리-세르치피카치-이호르스 --우제르-다타-지르=/호트/.콘피그/미크로조프트-에드지-데브/ --누-산드보스 & 지종
microsoft-edge-dev --disable-web-security --ignore-certificate-errors --user-data-dir=/root/.config/microsoft-edge-dev/ --no-sandbox & disown
미크로조프트-에드지-데브 --지자블리-웨브-세쿠리치 --이그노리-세르치피카치-이호르스 --우제르-다타-지르=/호트/.콘피그/미크로조프트-에드지-데브\_/ --누-산드보스 & 지종
microsoft-edge-dev --disable-web-security --ignore-certificate-errors --user-data-dir=/root/.config/microsoft-edge-dev\_/ --no-sandbox & disown

오브시지앙 --누-산드보스 & 지종
obsidian --no-sandbox & disown

---
---

에리'스 아 콤플레치 세켄시 오프 코만드스 타트 요 캉 우지 투 클레아누프 티 니슈스 스토라지, 스타르칭그 위트 티 니스-콜렉트-가르바지 -드 코만드:
Here's a complete sequence of commands that you can use to cleanup the NixOS storage, starting with the nix-collect-garbage -d command:

니스-콜렉트-가르바지 -드: 티스 코만드 콜렉트스 안드 델레치스 아니 가르바지 타트 이스 노트 베잉그 우제드 비 아니 악치비 제네라치옹 오프 티 시스텡, 안드 이트 위우 아우수 델레치 티 제네라치옹스 오프 티 시스텡 타트 아리 누 론제르 잉 우지. 티스 스테프 위우 프레이 우프 지스크 스파시 비 헤모빙그 오우드, 우누제드 파카지 베르시옹스 안드 오우드, 우누제드 시스텡 콘피구라치옹스.
nix-collect-garbage -d: This command collects and deletes any garbage that is not being used by any active generation of the system, and it will also delete the generations of the system that are no longer in use. This step will free up disk space by removing old, unused package versions and old, unused system configurations.

니스-스토리 --오프치미지: 티스 코만드 오프치미지스 티 스토리 비 콤팍칭그 아르드 링크스. 티스 스테프 위우 에우프 요 헤클라잉 지스크 스파시 비 헤모빙그 두플리카치스 오프 필리스 타트 아리 사레드 베트웽 무우치플리 파카지스.
nix-store --optimize: This command optimizes the store by compacting hard links. This step will help you reclaim disk space by removing duplicates of files that are shared between multiple packages.

니스-스토리 --베리피: 티스 코만드 베리피스 티 인테그리치 오프 티 스토리. 이트 셱스 타트 티 스토리 파트스 아리 발리드 안드 타트 아우 티 필리스 아리 프레젠트 안드 노트 코후프테드, 티스 스테프 위우 에우프 요 데텍트 아니 프로블렝 타트 마이 비 카우제드 비 아 브로켕 스토리 안드 인수리 티 시스텡 이스 스치우 워르킹그 코헥틀리.
nix-store --verify: This command verifies the integrity of the store. It checks that the store paths are valid and that all the files are present and not corrupted, this step will help you detect any problem that may be caused by a broken store and ensure the system is still working correctly.

니스-샤네우 --우프다치: 티스 코만드 우프다치스 요르 시스텡 샤네우스 안드 아우수, 이프 네웨르 베르시옹스 오프 파카지스 아리 아바일라블리, 이트 위우 돈로아드 안드 인스타우 텡. 티스 스테프 위우 에우프 요 투 케프 요르 시스텡 우프-투-다치 안드 헤두시 티 스파시 우제드 비 오우드 베르시옹스 오프 티 파카지스.
nix-channel --update: This command updates your system channels and also, if newer versions of packages are available, it will download and install them. This step will help you to keep your system up-to-date and reduce the space used by old versions of the packages.

이트'스 임포르탄트 투 케프 잉 민드 타트 테지 코만드스 아리 클레아닝그 티 스토라지 우제드 비 요르 쿠헨트 우제르, 테리 아리 오테르 와이스 투 클레앙 티 스토라지 우제드 비 아우 우제르스, 수시 아스 우징그 티 핀드 코만드 콤비네드 위트 티 니스-스토리 --델레치 코만드, 부트 이트'스 아우와이스 임포르탄트 투 비 카레풀 엥 우징그 테지 코만드스 아스 테이 마이 델레치 임포르탄트 필리스 안드 다타, 이트'스 아우와이스 아 고드 이데아 투 마키 수리 투 아비 아 바쿠프 오르 아 스나프소트 오프 요르 시스텡 베포리 후닝그 테지 코만드스.
It's important to keep in mind that these commands are cleaning the storage used by your current user, there are other ways to clean the storage used by all users, such as using the find command combined with the nix-store --delete command, but it's always important to be careful when using these commands as they may delete important files and data, it's always a good idea to make sure to have a backup or a snapshot of your system before running these commands.

이트'스 헤코멘데드 투 훙 테지 코만드스 잉 티 오르데르 이 멘치오네드, 아스 테이 아리 부이우징그 옹 이아시 오테르 투 클레아르 지페렌트 치페스 오프 스토라지 이수이스, 아우수, 셰크 요르 시스텡 스토라지 스파시 베포리 안드 아프테르 후닝그 테지 코만드스, 투 아비 아 클레아르 비에 오프 티 임팍트 오프 이아시 코만드 옹 요르 스토라지.
It's recommended to run these commands in the order I mentioned, as they are building on each other to clear different types of storage issues, also, check your system storage space before and after running these commands, to have a clear view of the impact of each command on your storage.
