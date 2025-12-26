function updateCountdown() {
    const now = new Date();

    const nextYear = now.getFullYear() + 1;
    const target = new Date(`${nextYear}-01-01T00:00:00`);
    const diff = target - now;

    // 動態更新標題
    document.querySelector('.title').textContent = `${nextYear} 新年倒數`;

    const timeOptions = { hour: '2-digit', minute: '2-digit', second: '2-digit', hour12: false };
    const dateOptions = { year: 'numeric', month: 'long', day: 'numeric', weekday: 'long' };

    document.getElementById('current-time').textContent = now.toLocaleTimeString('zh-TW', timeOptions);
    document.getElementById('current-date').textContent = now.toLocaleDateString('zh-TW', dateOptions);

    if (diff > 0) {
        const d = Math.floor(diff / (1000 * 60 * 60 * 24));
        const h = Math.floor((diff / (1000 * 60 * 60)) % 24);
        const m = Math.floor((diff / (1000 * 60)) % 60);
        const s = Math.floor((diff / 1000) % 60);

        document.getElementById('days').textContent = d.toString().padStart(2, '0');
        document.getElementById('hours').textContent = h.toString().padStart(2, '0');
        document.getElementById('minutes').textContent = m.toString().padStart(2, '0');
        document.getElementById('seconds').textContent = s.toString().padStart(2, '0');
    } else {
        document.querySelector('.title').textContent = `${nextYear} 新年快樂！`;
        document.querySelector('.countdown-grid').innerHTML = '<div style="grid-column: span 4; font-size: 2rem; font-weight: 800;">Happy New Year!</div>';
    }
    const startOfYear = new Date(now.getFullYear(), 0, 1);
    const endOfYear = new Date(now.getFullYear() + 1, 0, 1);

    // 更新進度條標題
    document.querySelector('.progress-info h3').textContent = `${now.getFullYear()} 年度完成度`;

    const totalYearTime = endOfYear - startOfYear;
    const elapsedYearTime = now - startOfYear;
    const progressPercentage = (elapsedYearTime / totalYearTime) * 100;

    const progressText = document.getElementById('progress-text');
    const percentVal = document.getElementById('percent-val');
    const progressBar = document.getElementById('progress-bar');

    const formattedPercent = progressPercentage.toFixed(6);
    progressText.textContent = formattedPercent + '%';
    percentVal.textContent = Math.floor(progressPercentage);

    const circumference = 2 * Math.PI * 45;
    const offset = circumference - (progressPercentage / 100) * circumference;
    progressBar.style.strokeDasharray = circumference;
    progressBar.style.strokeDashoffset = offset;
}

updateCountdown();
setInterval(updateCountdown, 1000);
