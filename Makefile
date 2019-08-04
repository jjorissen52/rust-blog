.PHONY: reflex
.DEFAULT_GOAL := reflex


reflex:
	reflex -r "(\.rs$\|\.hbs$\)" --start-service cargo run