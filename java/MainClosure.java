import java.util.*;
import java.util.stream.Stream;
import java.util.stream.StreamSupport;

public class MainClosure extends NixLazyBase {

	public static void main(String[] args) {
		System.out.println(new MainClosure().force().call(NixInteger.create(5)));
	}

	public NixValue force() {
		// let binding

		NixAttrset let = (NixAttrset) NixAttrset.create(new HashMap<>()).force();

		scopes = scopes.clone();
		scopes.push(let);

		final ArrayDeque<NixAttrset> scopes = this.scopes;
		final ArrayDeque<NixAttrset> withs = this.scopes;

		let.value.put("a", () -> findVariable(scopes, withs,"b").force());
		let.value.put("b", () -> NixInteger.create(5).force());

		NixValue returnValue = (arg) -> arg.add(findVariable(scopes, withs, "a")).force();

		this.scopes = scopes.clone();
		this.scopes.pop();

		return returnValue;
	}
}
