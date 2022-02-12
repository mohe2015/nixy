public class MainClosure implements NixObject {

	public NixObject call(NixObject arg) {
		if (arg == null) {
			throw new IllegalArgumentException("This is a lambda. Therefore you need to pass a parameter.");
		}
		return (arg2) -> {
			if (arg2 != null) {
				throw new IllegalArgumentException("This is a lazy value and no lambda. Therefore you need to pass null.");
			}
			return ((NixBoolean)NixBoolean.create(true).call(null)).value ?
					NixInteger.create(1).add(arg).call(null) :
					NixInteger.create(2).add(call(arg)).call(null);
		};
	}

	public static void main(String[] args) {
		System.out.println(new MainClosure().call(NixInteger.create(2)));
		System.out.println(new MainClosure().call(NixInteger.create(2)).call(null));
	}
}
