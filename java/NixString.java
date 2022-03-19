import java.util.Objects;

public class NixString implements NixValue, NixToString {

	String value;

	private NixString(String value) {
		this.value = value;
	}

	public static NixLazy create(String value) {
		return new NixLazy() {

			@Override
			public NixValue force() {
				return new NixString(value);
			}
		};
	}

	@Override
	public NixValue call(NixLazy arg) {
		throw new IllegalStateException("can't call a string");
	}

	@Override
	public boolean equals(Object o) {
		if (this == o) return true;
		if (o == null || getClass() != o.getClass()) return false;
		NixString nixString = (NixString) o;
		return Objects.equals(value, nixString.value);
	}

	@Override
	public int hashCode() {
		return Objects.hash(value);
	}

	@Override
	public String toString() {
		return "NixString{" +
				"value='" + value + '\'' +
				'}';
	}


	@Override
	public NixString toNixString() {
		return this;
	}
}
