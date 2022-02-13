import java.util.Objects;

public class NixFloat implements NixValue, NixNumber, NixToString {

	float value;

	private NixFloat(float value) {
		this.value = value;
	}

	public static NixLazy create(float value) {
		return () -> new NixFloat(value);
	}

	@Override
	public NixValue call(NixLazy arg) {
		throw new IllegalStateException("can't call a float");
	}

	@Override
	public boolean equals(Object o) {
		if (this == o) return true;
		if (o == null || getClass() != o.getClass()) return false;
		NixFloat that = (NixFloat) o;
		return value == that.value;
	}

	@Override
	public int hashCode() {
		return Objects.hash(value);
	}

	@Override
	public String toString() {
		return "NixFloat{" +
				"value=" + value +
				'}';
	}

	@Override
	public NixFloat toNixFloat() {
		return this;
	}

	@Override
	public NixString toNixString() {
		return (NixString) NixString.create(Float.toString(value)).force();
	}
}
