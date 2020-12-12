(function() {var implementors = {};
implementors["anyhow"] = [{"text":"impl AsRef&lt;dyn Error + 'static + Send + Sync&gt; for Error","synthetic":false,"types":[]},{"text":"impl AsRef&lt;dyn Error + 'static&gt; for Error","synthetic":false,"types":[]}];
implementors["bitvec"] = [{"text":"impl&lt;O, V&gt; AsRef&lt;BitSlice&lt;O, &lt;V as BitView&gt;::Store&gt;&gt; for BitArray&lt;O, V&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;O: BitOrder,<br>&nbsp;&nbsp;&nbsp;&nbsp;V: BitView + Sized,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;O, T, '_&gt; AsRef&lt;BitSlice&lt;O, T&gt;&gt; for Iter&lt;'_, O, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;O: BitOrder,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: BitStore,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;O, T&gt; AsRef&lt;BitSlice&lt;O, T&gt;&gt; for BitBox&lt;O, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;O: BitOrder,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: BitStore,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;O, T, '_&gt; AsRef&lt;BitSlice&lt;O, T&gt;&gt; for Drain&lt;'_, O, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;O: BitOrder,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: BitStore,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;O, T&gt; AsRef&lt;BitSlice&lt;O, T&gt;&gt; for BitVec&lt;O, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;O: BitOrder,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: BitStore,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["pulldown_cmark"] = [{"text":"impl&lt;'a&gt; AsRef&lt;str&gt; for InlineStr","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; AsRef&lt;str&gt; for CowStr&lt;'a&gt;","synthetic":false,"types":[]}];
implementors["regex_syntax"] = [{"text":"impl AsRef&lt;[u8]&gt; for Literal","synthetic":false,"types":[]}];
implementors["unicase"] = [{"text":"impl&lt;S:&nbsp;AsRef&lt;str&gt;&gt; AsRef&lt;str&gt; for Ascii&lt;S&gt;","synthetic":false,"types":[]},{"text":"impl&lt;S:&nbsp;AsRef&lt;str&gt;&gt; AsRef&lt;str&gt; for UniCase&lt;S&gt;","synthetic":false,"types":[]}];
implementors["wyz"] = [{"text":"impl&lt;T:&nbsp;Binary&gt; AsRef&lt;T&gt; for FmtBinary&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Display&gt; AsRef&lt;T&gt; for FmtDisplay&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;LowerExp&gt; AsRef&lt;T&gt; for FmtLowerExp&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;LowerHex&gt; AsRef&lt;T&gt; for FmtLowerHex&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Octal&gt; AsRef&lt;T&gt; for FmtOctal&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Pointer&gt; AsRef&lt;T&gt; for FmtPointer&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;UpperExp&gt; AsRef&lt;T&gt; for FmtUpperExp&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;UpperHex&gt; AsRef&lt;T&gt; for FmtUpperHex&lt;T&gt;","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()