pub mod graph;
/**
*  ✅ Satellites need positions before they can communicate → We need a basic orbital model to determine where they are.
   ✅ Routing & communication depend on knowing satellite locations → The graph structure must update dynamically.
   ✅ Security, storage, and messaging all rely on having a satellite network established.
*/
pub mod satellite;
pub mod tracking;
