# AStA Jobbörse API

Stellt eine API für die AStA Jobbörse bereit (https://asta.upb.de/jobboerse/).

Die Jobpostings werden in der Wordpress Datenbank aus der wp_posts Tabelle gelesen, wohin sie durch das "WP Job Manager" (https://wpjobmanager.com/) gelangt sind.

Geschrieben in Rust mit Rocket und Diesel


## Benutzung
Zum Benutzen muss die Umgebungsvariable `WP_DATABASE_URL` mit einer gültigen MySQL URL für eine Wordpress Installation mit dem Job Manager beschrieben sein (in der Form "mysql://user:pass@server.tld/database").

## Links
 - Rust (https://www.rust-lang.org/)
 - Rocket (https://rocket.rs/)
 - Diesel (https://diesel.rs/)
 - Wordpress (https://codex.wordpress.org/Main_Page)