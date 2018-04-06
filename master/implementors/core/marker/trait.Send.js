(function() {var implementors = {};
implementors["ralf"] = [{text:"impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"ralf/struct.State.html\" title=\"struct ralf::State\">State</a>&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,&nbsp;</span>",synthetic:true,types:["ralf::state::State"]},{text:"impl&lt;'a, 'b&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"ralf/struct.Action.html\" title=\"struct ralf::Action\">Action</a>&lt;'a, 'b&gt;",synthetic:true,types:["ralf::action::Action"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"ralf/struct.LoggingReducer.html\" title=\"struct ralf::LoggingReducer\">LoggingReducer</a>",synthetic:true,types:["ralf::reducer::LoggingReducer"]},{text:"impl&lt;'a, T&gt; !<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"ralf/struct.StateMachine.html\" title=\"struct ralf::StateMachine\">StateMachine</a>&lt;'a, T&gt;",synthetic:true,types:["ralf::state_machine::StateMachine"]},];

            if (window.register_implementors) {
                window.register_implementors(implementors);
            } else {
                window.pending_implementors = implementors;
            }
        
})()
