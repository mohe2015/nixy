public class IntegerAdditionClosure implements NixObject {

	static class IntegerAdditionClosure2 implements NixObject {
		NixInteger firstInt;

		public IntegerAdditionClosure2(NixInteger firstInt) {
			this.firstInt = firstInt;
		}

		@Override
		public NixObject call(NixObject arg) {
			if (arg instanceof NixInteger) {
				NixInteger secondInt = (NixInteger) arg;
				return firstInt.add(secondInt);
			} else {
				throw new UnsupportedOperationException();
			}
		}
	}

	public NixObject call(NixObject arg) {
		if (arg instanceof NixInteger) {
			NixInteger argInt = (NixInteger) arg;
			return new IntegerAdditionClosure2(argInt);
		} else {
			throw new UnsupportedOperationException();
		}
	}
}
