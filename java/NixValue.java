public interface NixValue {

	NixValue call(NixLazy arg);

	default NixValue callOrNop(NixLazy arg) {
		return this.call(arg);
	}

	default NixValue callOrNop() {
		return this;
	}
}
