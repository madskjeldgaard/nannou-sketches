(
	var port = 1211;

	s.waitForBoot{
		n = NetAddr.new("127.0.0.1", port);
	}
)
(
	Pdefn(\oscSender, 
		Pfunc({|ev|
			var arr;

			ev[\stream_name] = ev[\stream_name] ?? "stream1";
			ev[\freq] = ev.use({ ~freq.() });
			ev[\midinote] = ev.use({ ~midinote.() });
			ev[\degree] = ev.use({ ~degree.() });
			ev[\octave] = ev.use({ ~octave.() });

			ev[\dur] = ev.use({ ~dur.() });
			ev[\legato] = ev.use({ ~legato.() });

			ev[\pan] = ev.use({ ~pan.() });

			ev[\db] = ev.use({ ~db.() });
			ev[\amp] = ev.use({ ~amp.() });

			ev.keysValuesDo{|k, v| 
				arr = arr.add([ "/%".format(k), v ]);
			};

			ev[\targetAddr].sendBundle(0.0, *arr);
		})
	).copy(\send2);	

	Pdef(\sender1, 
		Pbind(
			\stream_name, "stream1",
			\dur, 0.125,
			\degree, Pwhite(0,10),
			\targetAddr, n,
			\pan, Pwhite(-1.0,1.0),
			\amp, Pwhite(0.25,0.9),
			\sendOSC, Pdefn(\oscSender) 
		)
	).play;

	Pdef(\sender2, 
		Pbindf(
			Pdef(\sender1),
			\stream_name, "stream2",
			\degree, Pwhite(0,10),
			\octave, 4,
			\pan, Pwhite(-1.0,1.0),
			\sendOSC, Pdefn(\send2) 
		)
	).play

)
