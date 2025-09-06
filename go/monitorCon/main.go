package main
import (
  "fmt"
  "strings"
  "regexp"
  "log"
  "net"
  gnet "github.com/shirou/gopsutil/v4/net"
)

/*
Done -> TLD rischiosi → .ru, .cn, .su, .xyz, .top, .club, .info, .tk, .pw, .gq, .cf, .ml, .zip, .mov, ecc.

Done -> Provider hosting generici → AWS, GCP, Azure, OVH, Hetzner, DigitalOcean, Cloudflare, Akamai, Fastly, Vercel, Netlify, Heroku. → da ricontrollare.


Done -> Hostname molto lungo → >60 caratteri = strano, >100 = molto sospetto.

Done -> Troppi subdomini → 4 o più livelli (a.b.c.d.example.com) → possibile infrastruttura offuscata.


Done -> Label randomizzata → sequenze tipo a9sd0f3k4jdl, molto numeri/lettere mescolati.


Done -> Domain squatting → nomi che imitano brand famosi con aggiunte (paypal-login.net, google-verify.xyz).

*/
var reRand = regexp.MustCompile(`^[a-z0-9-]{12,}$`)

var suspectTLD = []string{
    "ru",
    "cn",
    "su",
    "kp",
    "ir",
    "sy",
    "xyz",
    "top",
    "club",
    "work",
    "info",
    "click",
    "biz",
    "cc",
    "pw",
    "tk",
    "ml",
    "ga",
    "cf",
    "gq",
    "loan",
    "men",
    "mom",
    "cam",
    "zip",
    "mov",
    "live",
    "rest",
    "review",
    "stream",
    "date",
    "party",
    "science",
    "download",
    "fit",
    "skin",
    "kim",
}

var thirdPartyHosts = []string{
    "amazonaws",
    "cloudfront",
    "digitaloceanspaces",
    "ondigitalocean",
    "azure",
    "cloudapp",
    "windows",
    "googleusercontent",
    "1e100",
    "fastly",
    "akamaitechnologies",
    "akamai",
    "akamaiedge",
    "edgesuite",
    "cloudflare",
    "cdn.cloudflare",
    "vercel",
    "herokuapp",
    "netlify",
    "firebaseio",
    "linode",
    "vultrusercontent",
    "ovh",
    "hetzner",
}

var brandTargets = []string{
    "google", "gmail", "youtube", "facebook", "instagram", "whatsapp",
    "tiktok", "twitter", "linkedin", "snapchat", "github", "dropbox",
    "zoom", "slack", "microsoft", "office", "outlook", "hotmail", "live",
    "skype", "apple", "icloud", "spotify", "netflix", "paypal", "amazon",
    "ebay", "alibaba", "adobe", "visa", "mastercard", "stripe", "revolut",
    "wise", "skrill", "neteller", "venmo", "coinbase", "binance", "kraken",
    "bitfinex", "uber", "airbnb", "booking", "expedia", "wordpress", "shopify",
}

var phishingKeywords = []string{
	"login", "secure", "verify", "auth", "account", "support", "update",
	"pay", "payment", "billing", "bank",
}

func main() {

  conns, err := gnet.Connections("all")
  if err != nil { log.Fatal(err) }
  dnss := []string{}
  for _, c := range conns {
    rDNS := ""
    if c.Raddr.IP != "" {
      if names, _ := net.LookupAddr(c.Raddr.IP); len(names) > 0 {
        rDNS = names[0]
      }
    }
    if rDNS != "" {
      dnss = append(dnss, rDNS[:len(rDNS)-1])
    }

  }

  for _, dns := range dnss {
  	cal := severityCal(dns)
  	if cal <= 2 {
    	fmt.Printf("DNS: %s\nSeverity Points: %d\nValue: Ok\n\n", dns, cal)
	} else if cal >= 3 && cal <= 5 {
	    fmt.Printf("DNS: %s\nSeverity Points: %d\nValue: To Check\n\n", dns, cal)
	} else {
	    fmt.Printf("DNS: %s\nSeverity Points: %d\nValue: Suspect\n\n", dns, cal)
	}


  }


}

func severityCal(domain string) int {
   unpack := strings.Split(domain,".")
   severity := 0
   if TLDCheck(unpack[len(unpack)-1]) {
   	severity += 1
   }
   if thirdPartyHostsCheck(unpack[len(unpack) - 2]) {
   	severity += 1
   }
   hl := hostnameLenCheck(domain)
   if hl != 3{
   	severity += hl
   }

   if subdomainToLong(domain) {
   	severity += 1
   }
   severity += labelRandomCheck(domain)
   severity += classifyDomain(domain)
   return severity
}


func TLDCheck(tld string) bool {
	for _, stld := range suspectTLD {
		if tld == stld {
			return true
		}
	}
	return false
}

func thirdPartyHostsCheck(tphost string) bool {
	for _, thrh := range  thirdPartyHosts {
		if tphost == thrh {
			return true
		}
	}
	return false
}

func hostnameLenCheck(hlc string) int{
	lencheck := len(hlc)
	if lencheck < 60 {
		return 0
	}else if lencheck >= 60 {
		return 1
	}else if lencheck > 100 {
		return 2
	}
	return 3
}


func subdomainToLong(domain string) bool {
	lenSbCheck := len(strings.Split(domain, "."))
	if lenSbCheck > 4 {
		return true
	}
	return false
}

func labelRandomCheck(domain string) int {
	toCheck := strings.Split(domain, ".")
	severity := 0
	for _, toMa := range toCheck {
		if reRand.MatchString(toMa) {
			severity += 1
		}
	}
	return severity

}

func extractDomain(host string) string {
	parts := strings.Split(host, ".")
	if len(parts) < 2 {
		return host
	}
	return strings.Join(parts[len(parts)-2:], ".")
}

func classifyDomain(host string) int {
	domain := extractDomain(host)
	lower := strings.ToLower(domain)

	for _, h := range thirdPartyHosts {
		if strings.HasSuffix(lower, h) {
			return 1
		}
	}

	for _, b := range brandTargets {
		if strings.Contains(lower, b) {
			for _, k := range phishingKeywords {
				if strings.Contains(lower, k) {
					return 2
				}
			}
			return 1
		}
	}

	return 0
}

