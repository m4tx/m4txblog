use std::sync::LazyLock;

use indexmap::IndexMap;
use m4txblog_common::md_pages::MdPage;
use m4txblog_macros::md_page;

static POSTS: LazyLock<IndexMap<String, Vec<MdPage>>> = LazyLock::new(|| {
    let posts = vec![
        md_page!("2025-08-28-cot-v04-particularly-lazy"),
        md_page!("2025-07-08-towards-rust-web-best-errors"),
        md_page!("2025-05-13-cot-v03-even-lazier"),
        md_page!("2025-03-26-introducing-cot-02"),
        md_page!("2025-03-26-hello-world-yet-again"),
        md_page!("2025-02-18-welcome-cot-the-rust-web-framework-for-lazy-developers"),
        md_page!("2014-06-28-pl-urodziny_4"),
        md_page!("2014-05-07-google-code-in-2013-grand-prize-trip"),
        md_page!("2014-05-24-pl-google-code-in-2013-grand-prize-trip"),
        md_page!("2014-03-02-pl-google-code-in-2013"),
        md_page!("2014-01-01-pl-happy-new-2014-year"),
        md_page!("2013-10-25-pl-mroczne-sekrety-tworcow-gier-xi-minirelacja"),
        md_page!("2013-08-18-pl-m4txblog3"),
        md_page!("2013-08-17-pl-13milowka08-wspomnienia-i-wrazenia"),
        md_page!("2013-07-30-pl-less-czyli-lekarstwo-na-bolaczki-css"),
        md_page!("2013-07-20-pl-prawiek-i-inne-czasy-czyli-jedno-wielkie-wtf"),
        md_page!("2013-07-16-pl-avian-jvm-z-czym-to-sie-je"),
        md_page!("2013-07-02-pl-spoznione-trzecie-urodziny-m4txbloga"),
        md_page!("2013-04-16-pl-lte-internet-szybki-jak-diabli"),
        md_page!("2013-04-13-pl-mki-201213-final-finalu"),
        md_page!("2013-02-17-pl-smsprice-1-0-1"),
        md_page!("2013-02-07-pl-google-code-in-2012-nagrody"),
        md_page!(
            "2013-01-01-java-swt-to-awt-and-vice-versa-image-conversion-with-transparency-support"
        ),
        md_page!(
            "2013-01-01-pl-java-konwersja-obrazkow-swt-na-awt-i-vice-versa-z-przezroczystoscia"
        ),
        md_page!("2012-12-31-pl-hepi-nju-dwa-tysiace-trzynasty-jer"),
        md_page!("2012-12-10-pl-extcpp0x-pierwszy-dodatek-na-chromea"),
        md_page!("2012-12-08-pl-talent-technologia-tolerancja-czyli-zajecia-o-dziwnej-nazwie"),
        md_page!("2012-11-26-pl-google-code-in-2012-buuu"),
        md_page!("2012-10-28-pl-android-i-pierwsza-aplikacja-na-niego"),
        md_page!("2012-09-30-pl-przejscie-ze-swinga-na-swt"),
        md_page!("2012-09-03-pl-javafx-yyy-lol"),
        md_page!("2012-07-28-pl-jednorozce-i-pegazy-czyli-co-robi-programista-w-wolnym-czasie"),
        md_page!("2012-06-28-pl-2-rok-istnienia-m4txbloga"),
        md_page!("2012-06-26-pl-plissssssss-pomozcieeeee"),
        md_page!("2012-06-25-pl-lektor-w-filmach"),
        md_page!("2012-06-16-pl-carnobyl-nowa-gra"),
        md_page!("2012-05-04-pl-elektrownia-jadrowa-w-polsce"),
        md_page!("2012-04-08-pl-windows-8-consumer-preview-pierwsze-wrazenia"),
        md_page!("2012-04-01-pl-mario-fail-dzisiaj-nie-bedzie"),
        md_page!("2012-03-21-pl-mario-fail-lekki-poslizg"),
        md_page!("2012-03-14-pl-kolejna-gra-mario-fail"),
        md_page!("2012-03-06-pl-nagrody-z-google-code-in-2011"),
        md_page!("2012-03-04-pl-google-plus-pierwsze-wrazenia"),
        md_page!("2012-02-14-pl-swing-czyli-poradnik-jak-zrobic-beznadziejna-biblioteke-do-gui"),
        md_page!("2012-02-12-new-application-duplicate-lines-finder-2-0"),
        md_page!("2012-02-12-pl-duplicate-lines-finder-2-0"),
        md_page!("2012-01-14-pl-bezpieczenstwo-popularnych-protokolow-komunikacji"),
        md_page!(
            "2012-01-12-pl-minecraft-1-1-czyli-jak-denne-tlumaczenie-potrafi-przygotowac-spolecznosc"
        ),
        md_page!("2011-12-31-pl-happy-new-year"),
        md_page!("2011-11-30-pl-kilka-slow-na-temat-zachowania-teamu-hedgewars-na-google-code-in"),
        md_page!("2011-11-22-pl-google-code-in-2011-pierwsze-wykonane-zadanie"),
        md_page!("2011-11-19-pl-google-code-in-2011"),
        md_page!("2011-11-11-pl-nowa-gra-the-orb"),
        md_page!("2011-10-02-pl-ameryka-idioci"),
        md_page!("2011-09-08-pl-google-analytics-czyli-to-owo-na-temat-popularnosci-bloga"),
        md_page!("2011-08-30-pl-kurs-gtk-rozdzial-11"),
        md_page!("2011-08-18-pl-kurs-gtk-rozdzial-10"),
        md_page!("2011-08-17-pl-search-and-replace-nowa-prosta-aplikacja"),
        md_page!("2011-08-16-pl-kurs-gtk-rozdzial-9"),
        md_page!("2011-08-08-pl-kurs-gtk-rozdzial-8"),
        md_page!("2011-08-01-pl-kurs-gtk-rozdzial-7"),
        md_page!("2011-07-26-pl-kurs-gtk-rozdzial-6"),
        md_page!("2011-07-22-pl-kurs-gtk-rozdzial-5"),
        md_page!("2011-07-21-pl-kurs-gtk-rozdzial-4"),
        md_page!("2011-07-20-pl-kurs-gtk-rozdzial-3"),
        md_page!("2011-07-19-pl-kurs-gtk-rozdzial-2"),
        md_page!("2011-07-18-pl-kurs-gtk-rozdzial-1"),
        md_page!("2011-07-10-pl-premiera-mobilnego-smsprice"),
        md_page!("2011-06-30-pl-nowy-projekt-miner"),
        md_page!("2011-06-28-pl-m4txblog-to-juz-rok-minal"),
        md_page!("2011-06-24-pl-directx-vs-opengl-kolejna-porazka-microsoftu"),
        md_page!("2011-06-18-pl-firefox-5-yyy-lol"),
        md_page!("2011-06-08-pl-quake-live-pierwsze-wrazenia"),
        md_page!("2011-05-29-pl-ubuntu-11-04-pierwsze-wrazenia"),
        md_page!("2011-05-01-pl-najlepszy-darmowy-hosting"),
        md_page!("2011-04-14-pl-www-m4tx-pl"),
        md_page!("2011-04-14-pl-swiatlo-wolumetryczne-w-blenderze"),
        md_page!("2011-04-06-pl-discoverera-nie-bedzie-na-razie"),
        md_page!("2011-03-22-pl-m4txblog-2-0"),
        md_page!(
            "2011-03-12-pl-5-najlepszych-aplikacji-linuksowych-ktorych-nie-znajdziesz-na-windowsie"
        ),
        md_page!("2011-02-04-pl-discoverer-postepy"),
        md_page!("2011-01-20-pl-discoverer-czyli-magia-sandboksowych-gier"),
        md_page!("2011-01-08-pl-chromium-os-czy-warto"),
        md_page!("2010-12-18-pl-kilka-slow-o-wszechobecnym-spamie"),
        md_page!("2010-11-24-pl-zmiany-na-blogu-i-zapowiedz-mbota-0-03"),
        md_page!("2010-11-13-pl-co-microsoft-robi-z-ludzmi"),
        md_page!("2010-10-25-pl-o-google-slow-kilkanascie"),
        md_page!("2010-10-16-pl-pierwsze-kroki-z-linuksem-znowu-o-instalacji-programow"),
        md_page!("2010-10-10-pl-pierwsze-kroki-z-linuksem-sterowniki-gpu-zainstalowane"),
        md_page!("2010-10-09-pl-pierwsze-kroki-z-linuksem-xfire-dziala"),
        md_page!("2010-10-07-pl-pierwsze-kroki-z-linuksem-dzien-trzeci"),
        md_page!("2010-10-06-pl-pierwsze-kroki-z-linuksem-dzien-drugi"),
        md_page!("2010-10-05-pl-pierwsze-kroki-z-linuksem-instalacja"),
        md_page!("2010-09-15-pl-mbot-0-02-wydany"),
        md_page!("2010-09-14-pl-nigdy-wiecej-gadu-gadu"),
        md_page!("2010-09-07-pl-nigdy-wiecej-produktow-microsoftu"),
        md_page!("2010-08-30-pl-mbot"),
        md_page!("2010-08-27-pl-alternatywne-nazwy-przegladarek"),
        md_page!("2010-08-07-pl-zmiany"),
        md_page!("2010-08-03-pl-zmienne-i-stale-w-c"),
        md_page!("2010-07-23-pl-aktualizacje"),
        md_page!("2010-07-16-pl-premiera-programu-duplicate-lines-finder"),
        md_page!("2010-07-07-pl-premiera-programu-smsprice"),
        md_page!("2010-07-02-pl-funkcje-matematyczne-w-c-czesc-ii"),
        md_page!("2010-06-30-pl-funkcje-matematyczne-w-c-czesc-i"),
        md_page!("2010-06-29-pl-zapowiedz-programu-gbx-thumb-extractor"),
        md_page!("2010-06-28-hello-world-2"),
        md_page!("2010-06-28-pl-hello-world-2"),
    ];

    let mut map = IndexMap::new();
    for post in posts {
        map.entry(post.link.clone()).or_insert(vec![]).push(post);
    }
    map
});

static UNARCHIVED_POSTS: LazyLock<IndexMap<String, Vec<MdPage>>> = LazyLock::new(|| {
    let mut posts = POSTS.clone();
    posts.retain(|_, post_list| !post_list[0].is_archived());
    posts
});

static ARCHIVED_POSTS: LazyLock<IndexMap<String, Vec<MdPage>>> = LazyLock::new(|| {
    let mut posts = POSTS.clone();
    posts.retain(|_, post_list| post_list[0].is_archived());
    posts
});

pub fn get_post_map() -> &'static IndexMap<String, Vec<MdPage>> {
    &POSTS
}

pub fn get_unarchived_post_map() -> &'static IndexMap<String, Vec<MdPage>> {
    &UNARCHIVED_POSTS
}

pub fn get_archived_post_map() -> &'static IndexMap<String, Vec<MdPage>> {
    &ARCHIVED_POSTS
}
