import java.util.AbstractMap;
import java.util.List;
import java.util.Map;
import java.util.Objects;
import java.util.stream.Collectors;

public class NixAttrset implements NixValue {

	Map<String, NixLazy> value;

	public NixAttrset(Map<String, NixLazy> value) {
		this.value = value;
	}

	public static NixLazy create(Map<String, NixLazy> value) {
		return new NixLazy() {

			@Override
			public NixValue force() {
				return new NixAttrset(value);
			}
		};
	}

	@Override
	public NixValue call(NixLazy arg) {
		throw new IllegalStateException("can't call an attrset");
	}

	@Override
	public boolean equals(Object o) {
		if (this == o) return true;
		if (o == null || getClass() != o.getClass()) return false;
		NixAttrset that = (NixAttrset) o;
		return Objects.equals(value, that.value);
	}

	@Override
	public int hashCode() {
		return Objects.hash(value);
	}

	@Override
	public String toString() {
		return "NixAttrset{" +
				"value=" + value.entrySet().stream().map((e) -> new AbstractMap.SimpleEntry(e.getKey(), e.getValue().force())).collect(Collectors.toUnmodifiableList()) +
				'}';
	}
}
