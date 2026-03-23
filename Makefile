APPIMAGE_TOOL_URL = "https://github.com/AppImage/appimagetool/releases/download/continuous/appimagetool-x86_64.AppImage"

.PHONY: debug release appimage

debug:
	dotnet run

release:
	dotnet publish -c Release

clean:
	rm -rf AppImage

appimage: clean release
	mkdir -p AppImage/Authenticator.AppDir/usr/bin/
	mkdir -p AppImage/Authenticator.AppDir/usr/share/icons/
ifeq (,$(wildcard appimagetool))
	curl -o appimagetool -L $(APPIMAGE_TOOL_URL)
	chmod +x appimagetool
endif
	cp bin/Release/net10.0/linux-x64/publish/Authenticator AppImage/Authenticator.AppDir/usr/bin/
	cp Assets/Authenticator.png AppImage/Authenticator.AppDir/
	cp Assets/Authenticator.png AppImage/Authenticator.AppDir/usr/share/icons/
	cp Assets/Authenticator.desktop AppImage/Authenticator.AppDir/
	cp Assets/AppRun AppImage/Authenticator.AppDir/
	chmod +x AppImage/Authenticator.AppDir/AppRun
	chmod +x AppImage/Authenticator.AppDir/usr/bin/Authenticator
	./appimagetool AppImage/Authenticator.AppDir/ AppImage/Authenticator-x86_64.AppImage
