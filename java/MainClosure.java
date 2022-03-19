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

		let.value.put("a", () -> findVariable("b").force());
		let.value.put("b", () -> NixInteger.create(5).force());

		scopes.push(let);

		NixValue returnValue = (arg) -> arg.add(findVariable("a")).force();

		scopes.pop();

		return returnValue;
	}
}
