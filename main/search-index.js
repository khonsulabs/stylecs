var searchIndex = JSON.parse('{\
"stylecs":{"doc":"A specialized component system aimed at helping build a …","t":"IDDDDDDIYLLMLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLKLLMLLLLLLLLLOOLLLLLLLLLLLLLLLLLLLLLLLLLLLL","n":["DynamicComponent","Identifier","InvalidIdentifier","Iter","Name","StaticName","Style","StyleComponent","StyleComponent","authority","authority","authority","borrow","borrow","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","clone","clone","clone","clone_into","clone_into","clone_into","cmp","default","deref","eq","eq","eq","eq","fmt","fmt","fmt","fmt","fmt","fmt","from","from","from","from","from","from","from","get","get_by_name","get_or_default","hash","hash","inherited","inherited","inherited","inherited","inherited_from","into","into","into","into","into","into","into_iter","into_iter","into_iter","is_empty","iter","len","merge","merge","merge","merge","merged_with","name","name","name","name","new","new","new","next","partial_cmp","private","private","provide","push","static_name","style","to_name","to_owned","to_owned","to_owned","to_string","to_string","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","type_id","type_id","validate","with","with_capacity"],"q":["stylecs","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"d":["A style component that can be powered by data contained in …","A name that contains only <code>a-z</code>, <code>A-Z</code>, <code>0-9</code>, or <code>_</code> characters.","","An iterator over the components contained in a <code>Style</code>.","A globally unique name.","A statically defined <code>Name</code>.","A set of style components.","A style component. Implementors can be stored within <code>Style</code>.","","Returns the authority of this component. By default, this …","Returns the authority of this component. By default, this …","The unique name of the source of this name. For example, …","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","","Returns the argument unchanged.","Returns the style component of type <code>T</code>, if present.","Returns the style component of type <code>T</code>, if present.","Returns the style component of type <code>T</code>. If not present, …","","","Returns whether the component should be inherited. Affects …","Returns whether the component should be inherited. Affects …","Returns whether the component should be inherited. Affects …","Returns whether the component should be inherited. Affects …","Returns a new <code>Style</code>, merging the components of <code>self</code> with …","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","","Returns true if this style has no components.","Returns an iterator over the elements in this style.","Returns the number of components in this style.","Merges <code>self</code> with <code>other</code>, if it makes sense to do so for …","Merges <code>self</code> with <code>other</code>, if it makes sense to do so for …","Merges <code>self</code> with <code>other</code>, if it makes sense to do so for …","Merges <code>self</code> with <code>other</code>, if it makes sense to do so for …","Returns a new <code>Style</code>, merging the components of <code>self</code> with …","The unique name of this style component.","The unique name of this style component.","The unique name of this style component.","The locally unique name.","Validates <code>name</code> and returns an <code>Identifier</code> if name does not …","Returns a new <code>Name</code> using <code>authority</code> and <code>name</code>.","Returns a new style with no components.","","","Returns tne identifier used to designate a private …","Returns a new <code>Name</code> with <code>_</code> used as the authority.","","Adds a component to this style. Any existing values of the …","Returns a <code>StaticName</code>, which allows for a name to be …","A shorthand for creating a <code>Style</code> type from a compile-time …","Returns this static instance as a regular <code>Name</code>.","","","","","","","","","","","","","","","","","","","","","","","","","Validates <code>name</code> and returns an error if any invalid …","Adds a component to the style and returns it. Any existing …","Returns a new style with enough capacity to hold <code>capacity</code> …"],"i":[0,0,0,0,0,0,0,0,0,14,14,2,7,12,17,1,2,3,7,12,17,1,2,3,1,2,3,1,2,3,1,3,1,1,1,1,2,7,7,1,1,2,3,7,12,17,1,2,2,3,3,3,3,1,2,14,14,22,22,3,7,12,17,1,2,3,17,3,3,3,3,3,14,14,22,22,3,22,14,14,2,1,2,3,17,1,1,2,7,3,0,0,12,1,2,3,7,1,7,12,17,1,1,2,3,7,12,17,1,2,3,7,12,17,1,2,3,1,3,3],"f":[0,0,0,0,0,0,0,0,0,[[],1],[[],1],0,[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[1,1],[2,2],[3,3],[[]],[[]],[[]],[[1,1],4],[[],3],[1],[[1,5],6],[[1,5],6],[[1,1],6],[[2,2],6],[[7,8],[[10,[9]]]],[[7,8],[[10,[9]]]],[[1,8],11],[[1,8],11],[[2,8],11],[[3,8],11],[[]],[[]],[[]],[[]],[[]],[12,2],[[]],[3,13],[[3,2],[[13,[0]]]],[3,[[0,[14,15,16]]]],[1],[2],[[],6],[[],6],[[],6],[[],6],[[3,3],3],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[3],[3],[3,6],[3,17],[3,18],[[]],[[]],[[]],[[]],[[3,3],3],[[],2],[[],2],[[],2],0,[[[20,[[19,[5]]]]],[[10,[1,7]]]],0,[[],3],[17,13],[[1,1],[[13,[4]]]],[[],1],0,[21],[[3,[0,[22,16]]]],0,0,[12,2],[[]],[[]],[[]],[[],23],[[],23],[[],10],[[],10],[[],10],[[],10],[5,[[10,[1]]]],[[],10],[[],10],[[],10],[[],10],[[],10],[[],10],[[],10],[[],10],[[],24],[[],24],[[],24],[[],24],[[],24],[[],24],[5,[[10,[7]]]],[[3,[0,[22,16]]],3],[18,3]],"p":[[3,"Identifier"],[3,"Name"],[3,"Style"],[4,"Ordering"],[15,"str"],[15,"bool"],[3,"InvalidIdentifier"],[3,"Formatter"],[3,"Error"],[4,"Result"],[6,"Result"],[3,"StaticName"],[4,"Option"],[8,"StyleComponent"],[8,"Default"],[8,"Clone"],[3,"Iter"],[15,"usize"],[4,"Cow"],[8,"Into"],[3,"Demand"],[8,"DynamicComponent"],[3,"String"],[3,"TypeId"]]}\
}');
if (typeof window !== 'undefined' && window.initSearch) {window.initSearch(searchIndex)};
if (typeof exports !== 'undefined') {exports.searchIndex = searchIndex};
