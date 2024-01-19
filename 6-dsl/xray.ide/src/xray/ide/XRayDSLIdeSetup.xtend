/*
 * generated by Xtext 2.33.0
 */
package xray.ide

import com.google.inject.Guice
import org.eclipse.xtext.util.Modules2
import xray.XRayDSLRuntimeModule
import xray.XRayDSLStandaloneSetup

/**
 * Initialization support for running Xtext languages as language servers.
 */
class XRayDSLIdeSetup extends XRayDSLStandaloneSetup {

	override createInjector() {
		Guice.createInjector(Modules2.mixin(new XRayDSLRuntimeModule, new XRayDSLIdeModule))
	}
	
}
