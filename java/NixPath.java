import java.util.Objects;

public class NixPath implements NixValue, NixToString {

	String value;

	private NixPath(String value) {
		this.value = value;
	}

	public static NixLazy create(String value) {
		return () -> new NixPath(value);
	}

	@Override
	public NixValue call(NixLazy arg) {
		throw new IllegalStateException("can't call a path");
	}

	@Override
	public boolean equals(Object o) {
		if (this == o) return true;
		if (o == null || getClass() != o.getClass()) return false;
		NixPath nixString = (NixPath) o;
		return Objects.equals(value, nixString.value);
	}

	@Override
	public int hashCode() {
		return Objects.hash(value);
	}

	@Override
	public String toString() {
		return "NixPath{" +
				"value='" + value + '\'' +
				'}';
	}


	@Override
	public NixString toNixString() {
		// TODO FIXME add the path into the nix string so the dependency is tracked
		// maybe also do this in string concatenation because this should also work for derivation exprs
		return (NixString) NixString.create(this.value).force();
	}
}
