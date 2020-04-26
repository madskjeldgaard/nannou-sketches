(
	var port = 1211;

	TempoClock.default.tempo=1.5;

	s.waitForBoot{
		n = NetAddr.new("127.0.0.1", port);

		// An event type that sends some of the basic parameters (post
		// calculation) of a normal note event as osc bundles to the NetAddr in
		// the \targetAddr key
		Event.addEventType(\noteosc, {|server| 
			var arr; 
			var ev = currentEnvironment;

			~type = \note;

			~stream_name = ~stream_name ?? "stream1";
			~freq = ev.use({ ~freq.() });
			~midinote = ev.use({ ~midinote.()});
			~degree = ev.use({ ~degree.() });
			~octave = ev.use({ ~octave.() });

			~dur = ev.use({ ~dur.() });
			~legato = ev.use({ ~legato.() });

			~pan = ev.use({ ~pan.() });

			~db = ev.use({ ~db.() });
			~amp = ev.use({ ~amp.() });

			// Construct osc bundle from environment
			ev.keysValuesDo{|k, v| 
				arr = arr.add([ "/%".format(k), v ]);
			};

			~targetAddr.sendBundle(0.0, *arr);

			ev.play;
		});

		Pdef(\sender1, 
			Pbind(
				\type, \noteosc,
				\stream_name, "stream1",
				\scale, Scale.whole,
				\dur, 1/8,
				\degree, Pwhite(0,10),
				\targetAddr, n,
				\pan, Pwhite(0.0,1.0),
				\amp, Pwhite(0.25,0.9),
			)
		).play;

		Pdef(\sender2, 
			Pbind(
				\type, \noteosc,
				\stream_name, "stream2",
				\degree, Pwhite(0,10),
				\scale, Scale.whole,
				\dur, 1/8,
				\octave, 4.0,
				\targetAddr, n,
				\pan, Pwhite(-1.0,0.0),
				\amp, Pwhite(0.25,0.9),
			)
		).play;

		Pdef(\sender3, 
			Pbind(
				\type, \noteosc,
				\stream_name, "stream3",
				\degree, Pwhite(0,10),
				\scale, Scale.whole,
				\dur, Pxrand([3.0, 4.0, 2.0],inf),
				\octave, 3,
				\pan, Pwhite(-1.0,1.0),
				\targetAddr, n,
				\amp, Pwhite(0.25,0.9),
			)
		).play;
	}
)
