import java.util.IdentityHashMap;
import java.util.Map;

public class MainClosure implements NixLazy {

	public static void main(String[] args) {
		System.out.println(new MainClosure().force().call(NixInteger.create(5)));
	}

	public NixValue force() {

		return NixAttrset.create(new IdentityHashMap<String, NixLazy>() {{
			// head
			NixLazy a = NixInteger.create(5);
			NixLazy b = NixInteger.create(7);

			// body
			this.put("a".intern(), a);
			this.put("b".intern(), a);
		}}).force();
	}
}
