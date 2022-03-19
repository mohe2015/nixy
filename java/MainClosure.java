import java.util.*;
import java.util.stream.Stream;
import java.util.stream.StreamSupport;

public class MainClosure extends NixLazyBase {

	public static void main(String[] args) {
		System.out.println(new MainClosure().force().call(NixInteger.create(5)));
	}

	public NixValue force() {
		ArrayDeque<NixAttrset> scopes = new ArrayDeque<>();
		ArrayDeque<NixAttrset> withs = new ArrayDeque<>();

		scopes.push((NixAttrset) globals.force());

		// let binding

		NixAttrset let = (NixAttrset) NixAttrset.create(new HashMap<>()).force();

		scopes = scopes.clone();
		scopes.push(let);

		ArrayDeque<NixAttrset> finalScopes = scopes;
		let.value.put("a", () -> findVariable(finalScopes, withs,"b").force());
		let.value.put("b", () -> NixInteger.create(5).force());

		ArrayDeque<NixAttrset> finalScopes1 = scopes;
		NixValue returnValue = (arg) -> arg.add(findVariable(finalScopes1, withs, "a")).force();

		scopes = scopes.clone();
		scopes.pop();

		return returnValue;
	}
}
