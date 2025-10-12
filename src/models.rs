use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OsintTool {
    pub name: String,
    pub url: String,
    pub description: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OsintCategory {
    pub id: String,
    pub name: String,
    pub description: String,
    pub subcategories: Vec<OsintCategory>,
    pub tools: Vec<OsintTool>,
    pub parent_id: Option<String>,
}

impl OsintCategory {
    pub fn new(id: impl Into<String>, name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            description: description.into(),
            subcategories: Vec::new(),
            tools: Vec::new(),
            parent_id: None,
        }
    }

    pub fn with_subcategories(mut self, subcategories: Vec<OsintCategory>) -> Self {
        self.subcategories = subcategories;
        self
    }

    pub fn with_tools(mut self, tools: Vec<OsintTool>) -> Self {
        self.tools = tools;
        self
    }

    #[allow(dead_code)]
    pub fn with_parent(mut self, parent_id: impl Into<String>) -> Self {
        self.parent_id = Some(parent_id.into());
        self
    }

    pub fn has_children(&self) -> bool {
        !self.subcategories.is_empty() || !self.tools.is_empty()
    }

    #[allow(dead_code)]
    pub fn is_leaf(&self) -> bool {
        self.subcategories.is_empty()
    }
}

impl OsintTool {
    pub fn new(name: impl Into<String>, url: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            url: url.into(),
            description: description.into(),
            tags: Vec::new(),
        }
    }

    #[allow(dead_code)]
    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }
}

pub fn create_osint_categories() -> Vec<OsintCategory> {
    vec![
        // Maps, Geolocation and Transport
        OsintCategory::new(
            "maps_geo",
            "Maps, Geolocation and Transport",
            "Tools for geographic analysis and transportation research"
        ).with_subcategories(vec![
            OsintCategory::new("social_media_photos", "Social media and photos", "Social media geolocation tools")
                .with_tools(vec![
                    OsintTool::new("Apps.skylens.io", "https://app.skylens.io/", "Posts with geotags from five social networks at once on one map (Twitter, YouTube, Instagram, Flickr, Vkontakte)"),
                    OsintTool::new("photo-map.ru", "http://photo-map.ru/", "search geotagged photos from VK.com"),
                    OsintTool::new("Snapchat map", "https://map.snapchat.com/", "Snapchat location sharing map"),
                    OsintTool::new("YouTube Geofind", "https://mattw.io/youtube-geofind/", "view YouTube geotagged video on map"),
                    OsintTool::new("Flickr Photo Map", "https://www.flickr.com/map", "Flickr geotagged photos on map"),
                    OsintTool::new("I know where your cat lives", "https://iknowwhereyourcatlives.com/", "geotagged photo from Instagram with #cat hashtag"),
                    OsintTool::new("Trendsmap.com", "https://www.trendsmap.com/map", "Explore most popular Twitter trends, hashtags and users on the worldmap"),
                    OsintTool::new("Pastvu.com", "https://pastvu.com/", "View historical photos taken at a particular location on a map"),
                    OsintTool::new("BirdHunt", "https://birdhunt.co/", "A very simple tool that allows you to select a geolocation/radius and get a list of recent tweets made in that place"),
                    OsintTool::new("WikiShootMe", "https://wikishootme.toolforge.org/", "Worldwide map of geotagged Wikipedia Creative Commons Images"),
                    OsintTool::new("COPERNIX", "https://copernix.io/", "Worldwide map of geolocated Wikipedia articles"),
                    OsintTool::new("Huntel.io", "https://www.huntintel.io/", "get a list of links to Facebook/Instagram locations linked to geographic coordinates"),
                ]),
            OsintCategory::new("nature", "Nature", "Nature and environmental tracking tools")
                .with_tools(vec![
                    OsintTool::new("Map View NGMDB", "https://ngmdb.usgs.gov/mapview/", "map for exploring some geologic maps and articles from the NGMDB (National Geologic Map Database)"),
                    OsintTool::new("WAQI", "http://waqi.info", "World's Air Pollution: Real-time Air Quality Index map"),
                    OsintTool::new("GlobalFishingMap", "https://globalfishingwatch.org/map/", "click on a point on the map and get the data on the current fishing effort at that location"),
                    OsintTool::new("Lightning Maps", "https://www.lightningmaps.org/", "lightning strikes in real time and also data on thunderstorms that ended months or years ago"),
                    OsintTool::new("Light Pollution World Map", "https://www.lightpollutionmap.info/", "showing the degree of light pollution in different countries"),
                ]),
            OsintCategory::new("aviation", "Aviation", "Aviation and flight tracking tools")
                .with_tools(vec![
                    OsintTool::new("Skyvector", "https://skyvector.com/", "tool is designed for planning private flights. And you can find an incredible amount of data about the current situation in the sky on it"),
                    OsintTool::new("Flight Connections", "https://www.flightconnections.com/", "click on the airport on the map to see the cities from which it's possible fly direct"),
                    OsintTool::new("Globe.adsbexchange.com", "https://globe.adsbexchange.com/", "tracking flights on a map"),
                ]),
            OsintCategory::new("maritime", "Maritime", "Ship and maritime tracking tools")
                .with_tools(vec![
                    OsintTool::new("Track Trace", "https://www.track-trace.com/container", "tracking a shipping container by number"),
                    OsintTool::new("Container Tracking", "http://container-tracking.org/", "tracking a shipping container by number"),
                    OsintTool::new("The Shipping Database", "https://theshippingdatabase.com/", "comprehensive archive of the world ships. There is even data for 1820!"),
                    OsintTool::new("Submarine Cable Map", "https://www.submarinecablemap.com/", "submarine communications cables map"),
                ]),
            OsintCategory::new("railway", "Railway", "Railway and train tracking systems")
                .with_tools(vec![
                    OsintTool::new("Amtrak Status Maps Archive", "https://juckins.net/amtrak_status/archive/html/history.php", "find out the train schedule for a station on a particular day that passed many years ago (since 2008)"),
                    OsintTool::new("Live Train Map Russia", "https://rasp.yandex.ru/map/trains", "Live map of trains for Russia, Belarus, Ukraine, Kazakhstan and Abkhazia"),
                    OsintTool::new("Chronotrains", "https://chronotrains-eu.vercel.app/", "A terrific weekend travel planning service for Europeans. It shows how far you can go from a certain station in 5 hours by train"),
                ]),
        ]),

        // Social Media
        OsintCategory::new(
            "social_media",
            "Social Media",
            "Social media investigation and analysis tools"
        ).with_subcategories(vec![
            OsintCategory::new("twitter", "Twitter", "Twitter analysis tools")
                .with_tools(vec![
                    OsintTool::new("Stweet", "https://github.com/markowanga/stweet", "Opensource Python library for scraping tweets (by user, by hashtag, by keyword). NO LOGIN OR API KEY REQUIRED"),
                    OsintTool::new("BirdHunt", "https://birdhunt.co/", "A very simple tool that allows you to select a geolocation/radius and get a list of recent tweets made in that place"),
                    OsintTool::new("Twitter account detector", "https://chrome.google.com/webstore/detail/twitter-account-detector/papcdbgfejihdinhieggiamjnkclhkck/related", "A simple and fast Chrome extension that finds all Twitter accounts on a site"),
                    OsintTool::new("app.truthnest.com", "https://app.truthnest.com/", "best tool for Twitter account investigation"),
                    OsintTool::new("Whotwi", "https://en.whotwi.com/", "A free online tool for analysing your Twitter account: shows the mutual following; search for tweets by calendar; list of most active readers"),
                    OsintTool::new("Treeverse.app", "https://treeverse.app", "view dialogs in Twitter as a graph"),
                    OsintTool::new("Hashtagify", "https://hashtagify.me/", "compare the popularity of the two hashtags"),
                    OsintTool::new("Tweet Binder", "https://www.tweetbinder.com/", "detailed twitter account analyze"),
                ]),
            OsintCategory::new("youtube", "YouTube", "YouTube investigation tools")
                .with_tools(vec![
                    OsintTool::new("YouTube Whisperer", "https://huggingface.co/spaces/jeffistyping/Youtube-Whisperer", "Transcribe YouTube video"),
                    OsintTool::new("YouTube Unlisted Video", "https://unlistedvideos.com/", "search for videos available only by link on youtube"),
                    OsintTool::new("Noxinluencer", "https://noxinfluencer.com/youtube/channel-compare", "youtube channels comparing"),
                    OsintTool::new("YouTube comment Finder", "https://ytcomment.kmcat.uk/", "Find and analyze YouTube comments"),
                    OsintTool::new("YouTube Comment Downloader", "https://github.com/egbertbouman/youtube-comment-downloader", "easy to install and fast tool for downloading YouTube comments in txt/json. Does NOT require authorization or API keys"),
                    OsintTool::new("Montage.meedan.com", "https://montage.meedan.com", "Search YouTube video by date (uploaded or recording) and by geolocation"),
                ]),
            OsintCategory::new("tiktok", "TikTok", "TikTok analysis tools")
                .with_tools(vec![
                    OsintTool::new("Tiktok Timestamp", "https://bellingcat.github.io/tiktok-timestamp/", "determines the time of publication of the video to the nearest second. Just copy the link"),
                    OsintTool::new("TikStats", "https://tikstats.org/", "detailed statistics on the growth dynamics of subscribers, likes, and video views for the TikTok account"),
                    OsintTool::new("TikTok Scraper", "https://github.com/drawrowfly/tiktok-scraper", "scrapping video from user, trend or hashtag feed, extracting video's or user's metadata"),
                    OsintTool::new("Exolyt.com", "https://exolyt.com/", "TikTok profile analyze"),
                ]),
            OsintCategory::new("facebook", "Facebook", "Facebook investigation tools")
                .with_tools(vec![
                    OsintTool::new("Facebook People Directory", "https://www.facebook.com/directory/people/", "Facebook people search directory"),
                    OsintTool::new("sowdust.github.io/fb-search", "https://sowdust.github.io/fb-search/", "search facebook posts, people and groups using URL-filters"),
                    OsintTool::new("Facebook Latest Posts Scraper", "https://console.apify.com/actors/EtZ9lsiipPgKrQIi6", "Scrape Facebook posts with comments from one or multiple page URLs"),
                ]),
            OsintCategory::new("instagram", "Instagram", "Instagram analysis tools")
                .with_tools(vec![
                    OsintTool::new("search4faces.com/ig00", "https://search4faces.com/ig00/index.html", "reverse image face search by 1.7 millions Instagram avatars"),
                    OsintTool::new("Dumpor", "https://dumpoir.com/", "Online Instagram posts viewer. View and download Instagram posts, stories, profiles, followers, followings, tagged posts"),
                    OsintTool::new("Instagram location ID finder", "http://www.otzberg.net/iguess/json.php", "you know the name of the location, but you need to find its ID for use in osint tools"),
                    OsintTool::new("InstaLooter", "https://github.com/althonos/InstaLooter", "InstaLooter is a Python program that can download any picture or video associated with an Instagram profile, without any API access"),
                ]),
            OsintCategory::new("reddit", "Reddit", "Reddit analysis tools")
                .with_tools(vec![
                    OsintTool::new("Map of Reddit", "https://anvaka.github.io/map-of-reddit/", "an alternative format for interacting with Reddit"),
                    OsintTool::new("Reddit Investigator", "https://www.redditinvestigator.com/", "Reddit user investigation tool"),
                    OsintTool::new("redditcommentsearch.com", "http://redditcommentsearch.com", "getting a list of all comments by a Reddit user with a certain name"),
                    OsintTool::new("Reddit Scraper", "https://apify.com/trudax/reddit-scraper", "Crawl posts, comments, communities, and users without login"),
                    OsintTool::new("Redditsearch.io", "https://redditsearch.io/", "Reddit search tool"),
                    OsintTool::new("Camas Reddit Search", "https://camas.github.io/reddit-search/", "Search engines for Reddit with a lot of filters"),
                ]),
        ]),

        // Messengers
        OsintCategory::new(
            "messengers",
            "Messengers",
            "Messaging platform investigation tools"
        ).with_subcategories(vec![
            OsintCategory::new("telegram", "Telegram", "Telegram investigation tools")
                .with_tools(vec![
                    OsintTool::new("Telegago", "https://cse.google.com/cse?q=+&cx=006368593537057042503:efxu7xprihg", "Telegram search engine"),
                    OsintTool::new("Commentgram CSE", "https://cse.google.com/cse?cx=006368593537057042503:ig4r3rz35qi", "search by Telegram comments"),
                    OsintTool::new("Telegram Message Analyzer", "https://github.com/zqtay/Telegram-Message-Analyzer", "Export Telegram chat and get detailed analyze of it (message count, average message count per day, word frequency etc)"),
                    OsintTool::new("@SangMataInfo_bot", "https://t.me/SangMataInfo_bot", "forward a message from the user and find out the history of their name in Telegram"),
                    OsintTool::new("@tgscanrobot", "https://t.me/tgscanrobot", "telegram bot to show which telegram groups a person is member of"),
                    OsintTool::new("Telegram Nearby Map", "https://github.com/tejado/telegram-nearby-map", "Discover the location of nearby Telegram users on OpenStreetMap"),
                    OsintTool::new("Tgstat", "https://tgstat.com", "one of the largest directories of Telegram channels, which has detailed information about the growth of the audience"),
                ]),
            OsintCategory::new("whatsapp", "WhatsApp", "WhatsApp analysis tools")
                .with_tools(vec![
                    OsintTool::new("whatsanalyze.com", "https://Whatsanalyze.com", "analyzes WhatsApp group message statistics (world cloud, timeline, message frequency)"),
                    OsintTool::new("chatvisualizer.com", "https://chatvisualizer.com", "another WhatsApp chat analyzer"),
                    OsintTool::new("Watools.io", "https://watools.io/download-profile-picture", "download whatsapp profile picture"),
                    OsintTool::new("WAGSCRAPER", "https://github.com/riz4d/WaGpScraper", "Scraps Whatsapp Group Links From Google Results And Gives Working Links"),
                ]),
            OsintCategory::new("skype", "Skype", "Skype investigation tools")
                .with_tools(vec![
                    OsintTool::new("vedbex.com/tools/email2skype", "https://www.vedbex.com/tools/email2skype", "finding a Skype account by email"),
                    OsintTool::new("SkypeHunt", "https://github.com/8C/skypehunt", "A tool for finding Skype users by nickname. Shows a list of users with date of birth, year of account creation, country, avatar link"),
                ]),
        ]),

        // Domain/IP/Links
        OsintCategory::new(
            "domain_ip",
            "Domain/IP/Links",
            "Web infrastructure and domain analysis tools"
        ).with_subcategories(vec![
            OsintCategory::new("domain_investigation", "Domain/IP Investigation", "Domain and IP analysis tools")
                .with_tools(vec![
                    OsintTool::new("OWASP Amass", "https://github.com/OWASP/Amass", "The OWASP Amass Project performs network mapping of attack surfaces and external asset discovery using open source information gathering and active reconnaissance techniques"),
                    OsintTool::new("Investigator Recon Tool", "https://abhijithb200.github.io/investigator/", "web based handy-recon tool that uses different GoogleDorking techniques and some open sources service"),
                    OsintTool::new("Hakrawler", "https://github.com/hakluke/hakrawler", "Extreme(!) fast crawler designed for easy, quick discovery of links, endpoints and assets within a web application"),
                    OsintTool::new("Snyk.io", "https://snyk.io/test/website-scanner/", "Website Vulnerabilities Scanner"),
                ]),
            OsintCategory::new("website_analyze", "Website Analyze", "Website analysis and monitoring tools")
                .with_tools(vec![
                    OsintTool::new("AppSumo Content Analyzer", "https://app.buzzsumo.com/content/web", "Enter the name of the domain and find out for free its three most popular publications in social networks"),
                    OsintTool::new("OpenLinkProfiles", "http://openlinkprofiler.org/", "Get backlinks by website URL. Filter and sort backlinks by anchor, context, trust, LIS and industry"),
                    OsintTool::new("Lookyloo", "https://lookyloo.circl.lu/", "Webapp allowing to scrape a website and then displays a tree of domains calling each other"),
                    OsintTool::new("BGPView", "https://bgpview.io/", "web-browsing tool and an API that lets you gather information about the current state and structure of the internet"),
                ]),
        ]),

        // Image Search and Identification
        OsintCategory::new(
            "image_search",
            "Image Search and Identification",
            "Reverse image search and image analysis tools"
        ).with_subcategories(vec![
            OsintCategory::new("reverse_image_search", "Reverse Image Search Engines", "Reverse image search tools")
                .with_tools(vec![
                    OsintTool::new("News Myseldon", "https://news.myseldon.com/en/", "from the photo looks for famous and little-known (like minor officials) people"),
                    OsintTool::new("Ascii2d.net", "http://ascii2d.net", "Japanese reverse image search engine for anime lovers expose image properties, EXIF data"),
                    OsintTool::new("Searchbyimage.app", "https://searchbyimage.app/", "search clothes in online shops"),
                    OsintTool::new("Aliseeks.com", "https://www.aliseeks.com/search", "search items by photo in AliExpress and Ebay"),
                    OsintTool::new("IQDB.org", "https://iqdb.org/", "reverse image search specially for anime art"),
                    OsintTool::new("Same Energy", "https://same.energy/", "reverse image search engine for finding beautiful art and photos in the same style as the original picture"),
                    OsintTool::new("Revesearch.com", "https://revesesearch.com", "allows to upload an image once and immediately search for it in Google, Yandex, and Bing"),
                ]),
            OsintCategory::new("face_recognition", "Face Recognition and Search", "Face recognition and people search tools")
                .with_tools(vec![
                    OsintTool::new("Face Recognition", "https://github.com/ageitgey/face_recognition", "facial recognition api for Python and the command line"),
                    OsintTool::new("Search4faces.com", "https://search4faces.com/", "search people in VK, Odnoklassniki, TikTok and ClubHouse by photo or identikit"),
                    OsintTool::new("Telegram Facemath bot", "https://t.me/facematch_bot", "searching for a face among the archive of photographs from public events in Kazakhstan"),
                ]),
        ]),

        // Search Engines
        OsintCategory::new(
            "search_engines",
            "Search Engines",
            "Alternative search engines and search tools"
        ).with_subcategories(vec![
            OsintCategory::new("universal_search", "Universal Search Tools", "Multi-source search tools")
                .with_tools(vec![
                    OsintTool::new("S", "https://github.com/zquestz/s", "Search from command line in 106 different sources"),
                    OsintTool::new("searchall.net", "https://searchall.net", "75 fields for quick entry of queries to different search services on one page"),
                    OsintTool::new("Query-server", "https://query-server.herokuapp.com", "A tool that can send queries to popular search engines and return search results in JSON, CSV or XML format"),
                    OsintTool::new("Search Engines Scraper", "https://github.com/tasos-py/Search-Engines-Scraper", "Collects search results in text files. It's possible to search Google, Bing, DuckDuckGo, AOL and other search engines"),
                ]),
            OsintCategory::new("iot_search", "IOT Search Engines", "Internet of Things search engines")
                .with_tools(vec![
                    OsintTool::new("Netlas.io", "https://app.netlas.io/", "Search engine for every domain and host available on the Internet (like Shodan and Censys)"),
                    OsintTool::new("CriminalAPI", "https://www.criminalip.io/", "Search engine for all public IPs on the Internet"),
                    OsintTool::new("FullHunt", "https://fullhunt.io/", "Attack surface database of the entire Internet. Search info by domain, ip, technology, host, tag, port, city and more"),
                    OsintTool::new("Hunter", "https://hunter.how/", "Search engine for security researchers (analog Shodan, Censys, Netlas)"),
                ]),
        ]),

        // Additional Categories
        OsintCategory::new(
            "cryptocurrencies",
            "Cryptocurrencies",
            "Cryptocurrency investigation and blockchain analysis"
        ),

        OsintCategory::new(
            "people_search",
            "People Search",
            "People and person investigation tools"
        ),

        OsintCategory::new(
            "datasets",
            "Datasets",
            "Public datasets and data sources"
        ),

        OsintCategory::new(
            "archives",
            "Archives",
            "Web archives and historical data"
        ),
    ]
}
