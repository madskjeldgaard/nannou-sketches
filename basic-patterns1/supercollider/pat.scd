(
	var port = 1212;

	s.waitForBoot{
		n = NetAddr.new("127.0.0.1", port);
	}
)
(
	Pdef(\sender, 
		Pbind(
			\dur, Pwhite(0.075, 0.2), 
			// \degree, Pwhite(0,10), 
			\freq, Pwhite(80.0, 1500.0),
			\amp, Pwhite(0.1,0.25),
			\pan, Pwhite(-1.0,1.0),
			\sendOSC, Pfunc({|ev|
				ev.keysValuesDo{|k, v| 
					n.sendMsg("/%".format(k), v)
				};
			})
		)
	).play
)
