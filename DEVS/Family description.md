# Family description

- ["Proposal: Monorepo + workspace build for the esp-idf-* crates"](https://github.com/esp-rs/esp-idf-sys/issues/314) (GitHub Issues; Jun '24)

	..has the best description for what the roles of `esp-idf-svc`, `esp-idf-sys`, `embuild` etc. are
	
	In short, they are *so close to each other* that the author considered a monorepo, back in 2024. The same arguments still apply.

This clarified the author's mind on these things.

The `esp-idf-*` **work together** instead of merely dependening on each other.

>”Further, the reality is, NO useful end-user application of the esp-idf-* crates can be implemented without depending on the svc crate”

While `@ivmarkov` still doesn't provide facts to back this claim (this author can think of such a "useful end-user application"), it can now be seen as the author's intent, at least. :)

>”In retrospective - we could've renamed esp-idf-svc to just esp-idf […]”

i.e. it is the flag bearer, not unnecessarily layers above `esp-idf-sys` (or... is it? 😉).

