(
	var port = 1212;

	s.wa~~itForBoot{
		n = NetAddr.new("127.0.0.1", port);
	}
)
(
Pdef(\sender).clear;
	Pdef(\sender, 
		Pbind(
			\dur, Pwhite(0.075, 0.5), 
			// \degree, Pwhite(0,10), 
			\midinote, Pwhite(33,66),
			\freq, Pfunc({|ev|
				ev[\ctranspose] = ev[\ctranspose] ? 0.0;
				ev[\harmonic] = ev[\harmonic] ? 1.0;
				ev[\midinote] = ev[\midinote] ? 64;
					(ev[\midinote].value + ev[\ctranspose]).midicps * ev[\harmonic];
				}
			),
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
